// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://gitlab.com/TW0hank0/positive_mahjong
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::{self, net::TcpStream, sync, thread, time};

use iced::{
    self, Border, Color, Element, Length, Pixels, alignment, task,
    widget::{
        self, Column, Grid, Row, button, container, grid, scrollable, space, stack, text,
        text_input,
    },
};
use serde_json;
use tracing::{debug, error, info, trace, warn};
use tungstenite::{Message, WebSocket, connect};

use crate::{circular, easing};

use pmj_gamemodes;
use pmj_shared::shared::{self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");
pub const MATERIAL_SYMBOLS_OUTLINED: iced::font::Font =
    iced::font::Font::with_name("Material Symbols Outlined");

#[derive(Debug)]
pub struct Client {
    current_scene: ClientScenes,
    status_home: HomeStatus,
    status_play_base: PlayBaseStatus,
    ws: Option<sync::Arc<sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>>>,
    player_id: Option<u8>,
    theme: iced::theme::Theme,
    process_threads: Vec<ProThread>,
}

#[derive(Debug)]
pub struct ProThread {
    pub handle: thread::JoinHandle<ThreadResult>,
    pub start_time: time::Instant,
    pub process_type: ThreadProcessTypes,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientScenes {
    Home,
    PlayBase,
}

#[derive(Debug)]
pub struct HomeStatus {
    server_ip: String,
    try_connecting_server: bool,
    msgs: Vec<String>,
    connect_msg: Option<String>,
}

#[derive(Debug)]
pub struct PlayBaseStatus {
    server_ip: Option<String>,
    is_start: Option<bool>,
    hand_cards: Vec<pmj_gamemodes::base::shared::PMJCard>,
    game_msgs: Vec<String>,
    game_controller: PlayBaseController,
    current_turn: Option<u8>,
}

#[derive(Debug)]
pub enum PlayBaseController {
    NoCtrl,
    ThrowCard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIMessage {
    Home(HomeMessage),
    PlayBase(PlayBaseMessage),
    FetchThreadsStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
    VSoftKeyBoardInput(String),
    ConnectServer,
    SendFirstMsg,
    ReadFirstMsgResp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayBaseMessage {
    ReadWebsocketMsg,
    ThrowCard(pmj_gamemodes::base::shared::PMJCard),
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadResult {
    pub is_error: bool,
    pub result_read_first_msg_resp: Option<ThreadProcessResultReadFirstMsgResp>,
    pub result_play_base_read_websocket: Option<String>,
}

impl Default for ThreadResult {
    fn default() -> Self {
        Self {
            is_error: true,
            result_read_first_msg_resp: None,
            result_play_base_read_websocket: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadProcessResultReadFirstMsgResp {
    pub player_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadProcessTypes {
    ReadFirstMsgResp,
    PlayBaseReadWebsocket,
}

impl Client {
    pub fn new() -> Self {
        let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
        let _ = iced::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
        Self {
            current_scene: ClientScenes::Home,
            status_home: HomeStatus {
                server_ip: String::new(),
                try_connecting_server: false,
                msgs: Vec::new(),
                connect_msg: None,
            },
            status_play_base: PlayBaseStatus {
                server_ip: None,
                is_start: None,
                hand_cards: Vec::new(),
                game_controller: PlayBaseController::NoCtrl,
                game_msgs: Vec::new(),
                current_turn: None,
            },
            ws: None,
            player_id: None,
            theme: iced::theme::Theme::TokyoNight,
            process_threads: Vec::new(),
        }
    }
    pub fn update(&mut self, message: UIMessage) -> task::Task<UIMessage> {
        match message {
            UIMessage::FetchThreadsStatus => {
                if self.process_threads.len() > 0 {
                    trace!("Start fetch thread status...");
                    let mut rp_index = 0;
                    'loop_else: {
                        loop {
                            if rp_index >= self.process_threads.len() {
                                break;
                            } else {
                                debug!("rp_index={}", rp_index.clone());
                                let rpthread = self.process_threads.get(rp_index).unwrap();
                                if rpthread.handle.is_finished() {
                                    let _ = rpthread;
                                    let pthread = self.process_threads.remove(rp_index);
                                    match pthread.handle.join() {
                                        Ok(thread_result) => {
                                            if thread_result.is_error {
                                                error!("process_thread ran into error!");
                                                self.status_home.msgs.push(String::from(
                                                    "process_thread ran into error!",
                                                ));
                                                match pthread.process_type {
                                                    ThreadProcessTypes::ReadFirstMsgResp => {
                                                        return task::Task::done(UIMessage::Home(
                                                            HomeMessage::ReadFirstMsgResp,
                                                        ));
                                                    }
                                                    ThreadProcessTypes::PlayBaseReadWebsocket => {
                                                        return task::Task::done(
                                                            UIMessage::PlayBase(
                                                                PlayBaseMessage::ReadWebsocketMsg,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                trace!("process_thread finish sucessful.");
                                                match pthread.process_type {
                                                    ThreadProcessTypes::ReadFirstMsgResp => {
                                                        self.player_id = Some(
                                                            thread_result
                                                                .result_read_first_msg_resp
                                                                .unwrap()
                                                                .player_id,
                                                        );
                                                        debug!(
                                                            "ThreadProcessTypes::ReadFirstMsgResp => player_id -> {}",
                                                            self.player_id.clone().unwrap()
                                                        );
                                                        self.status_play_base.is_start =
                                                            Some(false);
                                                        self.current_scene = ClientScenes::PlayBase;
                                                        return iced::task::Task::done(
                                                            UIMessage::PlayBase(
                                                                PlayBaseMessage::ReadWebsocketMsg,
                                                            ),
                                                        );
                                                    }
                                                    ThreadProcessTypes::PlayBaseReadWebsocket => {
                                                        let msg_text = thread_result
                                                            .result_play_base_read_websocket
                                                            .unwrap();
                                                        let msg = serde_json::from_str::<pmj_gamemodes::base::shared::ServerMessageType>(
                                                            &msg_text
                                                        ).unwrap();
                                                        self.status_play_base.game_msgs.push(
                                                            serde_json::to_string_pretty(&msg)
                                                                .unwrap(),
                                                        );
                                                        match msg.msg_type {
                                                            pmj_gamemodes::base::shared::ServerMessageTypeKinds::GameStart => {
                                                                info!("收到伺服器遊戲開始訊息。");
                                                                self.status_play_base.is_start = Some(true);
                                                            }
                                                            pmj_gamemodes::base::shared::ServerMessageTypeKinds::GetCard => {
                                                                let got_card = msg.info_get_card.unwrap();
                                                                info!("取得卡牌：{:?}", got_card);
                                                                self.status_play_base.game_msgs.push(format!("取得卡牌：{:?}", got_card));
                                                                self.status_play_base.game_controller=PlayBaseController::ThrowCard;
                                                            }
                                                            pmj_gamemodes::base::shared::ServerMessageTypeKinds::HandCardChange => {
                                                                let handcard = msg.info_hand_card_change.unwrap();
                                                                self.status_play_base.hand_cards = handcard.clone();
                                                                debug!("手牌變動：{:#?}", handcard.clone());
                                                                self.status_play_base.game_msgs.push(format!("手牌變動：{:?}", handcard));
                                                            }
                                                            pmj_gamemodes::base::shared::ServerMessageTypeKinds::ChangedTurn => {
                                                                self.status_play_base.current_turn = msg.info_change_turn;
                                                            }
                                                            pmj_gamemodes::base::shared::ServerMessageTypeKinds::Error => {
                                                                error!("ServerMessageTypeKinds::Error => {:?}", msg.info_error);
                                                            }
                                                            _ => {
                                                                todo!("need to do.");
                                                            }
                                                        }
                                                        return iced::Task::done(
                                                            UIMessage::PlayBase(
                                                                PlayBaseMessage::ReadWebsocketMsg,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("thread: {:?}", e);
                                            self.status_home.msgs.push(format!("thread: {:?}", e));
                                        }
                                    }
                                } else {
                                    if rpthread.start_time.elapsed() > time::Duration::from_secs(60)
                                    {
                                        let _pthread = self.process_threads.remove(rp_index);
                                        return task::Task::done(UIMessage::Home(
                                            HomeMessage::ReadFirstMsgResp,
                                        ));
                                    }
                                    trace!("thread is not finish, rp_index={}", rp_index.clone());
                                    rp_index += 1;
                                }
                            }
                        }
                        return iced::task::Task::done(UIMessage::FetchThreadsStatus);
                    }
                } else {
                    debug!("process_threads.len() = {}", self.process_threads.len());
                }
            }
            UIMessage::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => {
                    if self.status_home.try_connecting_server {
                        self.status_home
                            .msgs
                            .push(String::from("已有正在嘗試連接的伺服器！"));
                    } else {
                        self.status_home.server_ip = server_ip;
                    }
                }
                HomeMessage::VSoftKeyBoardInput(key) => {
                    if self.status_home.try_connecting_server {
                        self.status_home
                            .msgs
                            .push(String::from("已有正在嘗試連接的伺服器！"));
                    } else {
                        if key == String::from("backspace") || key == String::from("\u{e14a}") {
                            self.status_home.server_ip.pop();
                        } else {
                            self.status_home.server_ip.push_str(&key);
                        }
                    }
                }
                HomeMessage::ConnectServer => {
                    if self.status_home.server_ip.is_empty() {
                        self.status_home
                            .msgs
                            .push(String::from("未輸入伺服器地址！"));
                    } else if self.status_home.try_connecting_server {
                        self.status_home
                            .msgs
                            .push(String::from("已有正在嘗試連接的伺服器！"));
                    } else {
                        self.status_home.try_connecting_server = true;
                        self.status_play_base.server_ip = Some(self.status_home.server_ip.clone());
                        let value = self.home_connect_server();
                        return value;
                    }
                }
                HomeMessage::SendFirstMsg => {
                    self.status_home.connect_msg = Some(String::from("正在傳送初連接訊息.."));
                    let value = self.home_send_first_msg();
                    self.status_home.connect_msg = Some(String::from("已傳送初連接訊息。"));
                    return value;
                }
                HomeMessage::ReadFirstMsgResp => {
                    self.status_home.connect_msg = Some(String::from("正在讀取初連接回覆.."));
                    self.process_threads.push(self.home_read_first_msg_resp());
                    return task::Task::done(UIMessage::FetchThreadsStatus);
                }
            },
            UIMessage::PlayBase(play_base_message) => match play_base_message {
                PlayBaseMessage::ReadWebsocketMsg => {
                    self.process_threads.push(self.play_base_read_websocket());
                    return iced::task::Task::done(UIMessage::FetchThreadsStatus);
                }
                PlayBaseMessage::ThrowCard(card) => {
                    // TODO
                    let msg =
                        serde_json::to_string(&pmj_gamemodes::base::shared::ClientMessageType {
                            msg_type:
                                pmj_gamemodes::base::shared::ClientMessageTypeKinds::GameAction,
                            info_game_action: Some(
                                pmj_gamemodes::base::shared::GameTurnTypes::ThrowCard,
                            ),
                            info_throw_card: Some(card),
                            ..Default::default()
                        })
                        .unwrap();
                    match write_reply(msg, self.ws.clone().unwrap()) {
                        Ok(_) => {}
                        Err(e) => {
                            error!("回覆失敗：{:?}", e);
                        }
                    }
                }
            },
        };
        return task::Task::none();
    }

    pub fn view(&self) -> Element<'_, UIMessage, iced::Theme, iced::Renderer> {
        let mut layout: Column<'_, UIMessage, iced::Theme, iced::Renderer> = Column::new()
            .align_x(alignment::Horizontal::Left)
            .padding(10);
        //
        match self.current_scene {
            ClientScenes::Home => {
                let mut layout_home = Column::new();
                // 標題欄
                {
                    let mut title_bar = Row::new().align_y(alignment::Vertical::Center);
                    title_bar = title_bar.push(
                        text(format!("{}", shared::PROJECT_NAME))
                            .height(Length::Shrink)
                            .size(Pixels::from(26)),
                    );
                    title_bar = title_bar.spacing(25);
                    title_bar = title_bar.push(
                        text(format!("v{}", shared::PROJECT_VERSION))
                            .height(Length::Shrink)
                            .size(Pixels::from(22)),
                    );
                    layout_home = layout_home.push(title_bar);
                }
                layout_home = layout_home.push(space().height(5));
                // 伺服器地址輸入處理
                {
                    let mut server_ip_input_bar = Row::new();
                    server_ip_input_bar = server_ip_input_bar
                        .push(
                            text_input("輸入伺服器地址...", &self.status_home.server_ip)
                                .on_input(|content| {
                                    UIMessage::Home(HomeMessage::InputServerIpChanged(content))
                                })
                                .on_submit(UIMessage::Home(HomeMessage::ConnectServer))
                                .size(Pixels::from(24))
                                .line_height(text::LineHeight::Relative(1.5))
                                .style(|t: &iced::Theme, s: text_input::Status| {
                                    // let p = t.extended_palette();
                                    let mut style = text_input::default(t, s);
                                    style.border.radius = iced::border::radius(6);
                                    style
                                }),
                        )
                        .spacing(15);
                    server_ip_input_bar = server_ip_input_bar.push(
                        button(text("連線").size(24))
                            .on_press(UIMessage::Home(HomeMessage::ConnectServer))
                            .style(rounded_primary_button),
                    );
                    layout_home = layout_home.push(server_ip_input_bar).spacing(35);
                }
                // 虛擬鍵盤
                {
                    let mut vsoft_keyboard = Grid::new()
                        .height(grid::Sizing::EvenlyDistribute(Length::Shrink))
                        .columns(8);
                    for key in 0..=9 {
                        vsoft_keyboard = vsoft_keyboard
                            .push(self.home_create_vsoft_key(format!("{}", key)))
                            .spacing(10);
                    }
                    for key in ALPHABET {
                        vsoft_keyboard = vsoft_keyboard
                            .push(self.home_create_vsoft_key(format!("{}", key).to_lowercase()))
                            .spacing(10);
                    }
                    for key in [":", "[", "]", ".", "/", "backspace"] {
                        vsoft_keyboard = vsoft_keyboard
                            .push(self.home_create_vsoft_key(format!("{}", key)))
                            .spacing(10);
                    }
                    layout_home = layout_home.push(vsoft_keyboard);
                }
                layout_home = layout_home.push(space().height(20));
                // 訊息顯示
                {
                    let mut msg_area = Column::new();
                    msg_area = msg_area.spacing(5);
                    let mut msg_number: u64 = 1;
                    for msg in self.status_home.msgs.iter() {
                        let ex_palette = self.theme.extended_palette();
                        let mut msg_row = Row::new();
                        msg_row = msg_row.spacing(3);
                        msg_row = msg_row.push(text(msg_number.to_string()).size(20).style(
                            move |_theme| text::Style {
                                color: Some(ex_palette.secondary.strong.text),
                            },
                        ));
                        msg_row = msg_row.spacing(3);
                        msg_row =
                            msg_row.push(text(msg).size(20).style(move |_theme| text::Style {
                                color: Some(ex_palette.secondary.base.text),
                            }));
                        msg_row = msg_row.spacing(3);
                        msg_area = msg_area.push(msg_row).spacing(1);
                        msg_number += 1;
                    }
                    layout_home = layout_home.push(container(msg_area).style(
                        |theme: &iced::theme::Theme| {
                            let ex_palette = theme.extended_palette();
                            let mut style = container::Style::default();
                            style.background =
                                Some(iced::Background::Color(ex_palette.secondary.base.color));
                            style.border = Border {
                                color: ex_palette.secondary.strong.color,
                                width: 2.5,
                                radius: iced::border::Radius::new(Pixels::from(8)),
                            };
                            style
                        },
                    ));
                }
                layout_home = layout_home.push(space().height(5));
                //
                if self.status_home.try_connecting_server {
                    let mut content_column = Column::new().padding(5).spacing(2);
                    content_column = content_column.push(
                        circular::Circular::new()
                            .easing(&easing::STANDARD)
                            .size(54.0),
                    );
                    content_column = content_column
                        .push(
                            text("連線中...")
                                .size(28)
                                .style(move |theme: &iced::theme::Theme| {
                                    let ex_palette = theme.extended_palette();
                                    text::Style {
                                        color: Some(ex_palette.secondary.base.text),
                                    }
                                })
                                .align_x(alignment::Horizontal::Center),
                        )
                        .spacing(2);
                    content_column = content_column.push(
                        text(
                            self.status_home
                                .connect_msg
                                .clone()
                                .unwrap_or(String::from("none")),
                        )
                        .style(|_t: &iced::Theme| text::Style {
                            color: Some(iced::Color::from_rgb8(56, 56, 56)),
                        })
                        .size(22),
                    );
                    let content = container(container(content_column).style(
                        move |theme: &iced::theme::Theme| {
                            let ex_palette = theme.extended_palette();
                            let mut style = container::Style::default();
                            style = style.background(iced::Background::Color(
                                ex_palette.secondary.weak.color,
                            ));
                            style.border(
                                Border::default()
                                    .color(ex_palette.secondary.strong.color)
                                    .rounded(12)
                                    .width(3),
                            );
                            style
                        },
                    ))
                    .center(Length::Fill)
                    .align_x(alignment::Alignment::Center)
                    .align_y(alignment::Alignment::Center)
                    .style(move |_theme| {
                        let mut style = container::Style::default();
                        style = style.background(iced::Background::Color(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.4,
                        }));
                        style = style.border(Border::default().width(0));
                        style
                    });
                    layout_home =
                        widget::column([
                            stack([scrollable(layout_home).into(), content.into()]).into()
                        ]);
                    layout = layout.push(layout_home);
                } else {
                    layout = layout.push(scrollable(layout_home));
                }
            }
            ClientScenes::PlayBase => {
                let mut layout_play_base = Column::new();
                {
                    let mut info_bar = Row::new().padding(iced::Padding::new(8.0));
                    info_bar = info_bar.push(text(format!(
                        "伺服器地址：{}",
                        self.status_play_base.server_ip.clone().unwrap(),
                    )));
                    info_bar = info_bar.push(space().width(Length::from(14)));
                    info_bar =
                        info_bar.push(text(format!("玩家識別碼：{}", self.player_id.unwrap())));
                    if self.status_play_base.is_start.unwrap_or(false) {
                        info_bar = info_bar.push(space().width(10)).push(text(format!(
                            "目前回合：{}",
                            if self.status_play_base.current_turn.is_some() {
                                format!("{}", self.status_play_base.current_turn.unwrap())
                            } else {
                                format!("{:?}", self.status_play_base.current_turn)
                            }
                        )));
                    }
                    layout_play_base = layout_play_base.push(info_bar)
                }
                if !self.status_play_base.is_start.unwrap() {
                    let mut status_bar = Column::new();
                    status_bar = status_bar.push(
                        text("等待遊戲開始")
                            .size(30)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    );
                    layout_play_base = layout_play_base.push(status_bar);
                } else {
                    {
                        let mut ctr_bar = Row::new();
                        let mut msg_bar = Column::new().width(Length::FillPortion(2));
                        let mut msg_num: u16 = 1;
                        for msg in self.status_play_base.game_msgs.iter() {
                            msg_bar = msg_bar
                                .push(text(msg_num.to_string()).size(14).style(
                                    |t: &iced::Theme| {
                                        let p = t.extended_palette();
                                        text::Style {
                                            color: Some(p.primary.base.text),
                                        }
                                    },
                                ))
                                .push(space().width(15))
                                .push(text(msg.clone()).size(14));
                            msg_num += 1;
                        }
                        ctr_bar = ctr_bar.push(msg_bar);
                        let mut card_bar = Column::new().width(Length::FillPortion(2));
                        for card in self.status_play_base.hand_cards.iter() {
                            card_bar = card_bar
                                .push(space().height(5))
                                .push(container(Row::new().width(Length::Fill).height(80)
                                    .push(
                                        text(format!("{}", match card.card_type {
                                            pmj_gamemodes::base::shared::PMJCardType::Dots => { card.info_dots.clone().unwrap().to_string()}
                                            pmj_gamemodes::base::shared::PMJCardType::Flower => {card.info_flower.clone().unwrap().to_string()}
                                            pmj_gamemodes::base::shared::PMJCardType::Line => { card.info_line.clone().unwrap().to_string() }
                                            pmj_gamemodes::base::shared::PMJCardType::TenThousand => { card.info_ten_thousand.clone().unwrap().to_string()}
                                            pmj_gamemodes::base::shared::PMJCardType::Words => { card.info_words.clone().unwrap().to_string()}
                                        })).size(24),
                                    )
                                    .push(space().width(5))
                                    .push(
                                        text(format!("{}", match card.card_type {
                                            pmj_gamemodes::base::shared::PMJCardType::Dots => {"筒"}
                                            pmj_gamemodes::base::shared::PMJCardType::Flower => {"花"}
                                            pmj_gamemodes::base::shared::PMJCardType::Line => {"條"}
                                            pmj_gamemodes::base::shared::PMJCardType::TenThousand => {"萬"}
                                            pmj_gamemodes::base::shared::PMJCardType::Words => {"字"}
                                        })).size(18)
                                    )
                                    .push(
                                        text(format!("第 {} 張", card.card_id.clone())).width(Length::Fill).size(15).align_x(alignment::Horizontal::Right)
                                    )).style(|t:&iced::Theme| {
                                        let p = t.extended_palette();
                                        let mut style = container::Style::default();
                                        style.border.radius = iced::border::Radius::new(10);
                                        style.border.width = 1.2;
                                        style.border.color = p.background.weak.color;
                                        style.text_color = Some(p.background.base.text);
                                        style.background = Some(iced::Background::Color(iced::Color::TRANSPARENT));
                                        style
                                    }));
                        }
                        ctr_bar = ctr_bar.push(card_bar);
                        layout_play_base = layout_play_base
                            .push(scrollable(ctr_bar).height(Length::FillPortion(2)));
                    }
                    // 玩家操作
                    {
                        let mut controller_bar = Column::new();
                        match self.status_play_base.game_controller {
                            PlayBaseController::NoCtrl => {}
                            PlayBaseController::ThrowCard => {
                                controller_bar = controller_bar.push(text("選擇一張你要丟的牌"));
                                let mut card_bar_elements: Vec<iced::Element<'_, UIMessage>> =
                                    Vec::new();
                                for card in self.status_play_base.hand_cards.iter() {
                                    card_bar_elements.push(
                                        button(
                                            Column::new().width(120)
                                            .height(160)
                                                .push(
                                                    text(format!("{}", match card.card_type {
                                                        pmj_gamemodes::base::shared::PMJCardType::Dots => { card.info_dots.clone().unwrap().to_string()}
                                                        pmj_gamemodes::base::shared::PMJCardType::Flower => {card.info_flower.clone().unwrap().to_string()}
                                                        pmj_gamemodes::base::shared::PMJCardType::Line => { card.info_line.clone().unwrap().to_string() }
                                                        pmj_gamemodes::base::shared::PMJCardType::TenThousand => { card.info_ten_thousand.clone().unwrap().to_string()}
                                                        pmj_gamemodes::base::shared::PMJCardType::Words => { card.info_words.clone().unwrap().to_string()}
                                                    })).size(24),
                                                )
                                                .push(
                                                    text(format!("{}", match card.card_type {
                                                        pmj_gamemodes::base::shared::PMJCardType::Dots => {"筒"}
                                                        pmj_gamemodes::base::shared::PMJCardType::Flower => {"花"}
                                                        pmj_gamemodes::base::shared::PMJCardType::Line => {"條"}
                                                        pmj_gamemodes::base::shared::PMJCardType::TenThousand => {"萬"}
                                                        pmj_gamemodes::base::shared::PMJCardType::Words => {"字"}
                                                    })).size(18)
                                                )
                                                .push(
                                                    text(format!("第 {} 張", card.card_id.clone())).height(Length::Fill).align_y(alignment::Vertical::Bottom).size(15).align_x(alignment::Horizontal::Right)
                                                )
                                        )
                                        .on_press(UIMessage::PlayBase(PlayBaseMessage::ThrowCard(
                                            card.clone(),
                                        )))
                                        .style(|t: &iced::Theme, s: button::Status| {
                                            let p = t.extended_palette();
                                            let mut style = button::Style::default();
                                            style.border.width = 1.2;
                                            style.border.radius = iced::border::radius(10);
                                            style.text_color = p.background.base.text;
                                            match s {
                                                button::Status::Active => {
                                                    style.border.color = p.background.strong.color;
                                                    style.background = None;
                                                }
                                                button::Status::Disabled => {
                                                    style.background =
                                                        Some(iced::Background::Color(
                                                            p.background.weak.color,
                                                        ));
                                                }
                                                button::Status::Hovered => {
                                                    style.border.color = p.primary.weak.color;
                                                    style.border.width = 1.5;
                                                }
                                                button::Status::Pressed => {
                                                    style.border.color = p.primary.strong.color;
                                                    style.border.width = 0.7;
                                                    style.border.radius = iced::border::radius(6);
                                                }
                                            }
                                            style
                                        })
                                        .into(),
                                    );
                                }
                                let mut card_bar_y = Column::new().spacing(3).padding(5);
                                let mut card_bar_x = Row::new().spacing(3).padding(5);
                                let mut card_bar_count = 1;
                                for e in card_bar_elements {
                                    card_bar_x = card_bar_x.push(e);
                                    card_bar_count += 1;
                                    if card_bar_count > 9 {
                                        card_bar_y = card_bar_y.push(card_bar_x);
                                        card_bar_x = Row::new().spacing(3).padding(5);
                                        card_bar_count = 1;
                                    }
                                }
                                card_bar_y = card_bar_y.push(card_bar_x);
                                controller_bar = controller_bar.push(card_bar_y);
                            }
                        }
                        layout_play_base = layout_play_base.push(
                            scrollable(
                                container(controller_bar)
                                    .style(|t: &iced::Theme| {
                                        let p = t.extended_palette();
                                        let mut style = container::Style::default();
                                        style.border = iced::Border {
                                            color: p.primary.base.color,
                                            width: 1.0,
                                            radius: iced::border::Radius::new(4),
                                        };
                                        style
                                    })
                                    .height(Length::FillPortion(3)),
                            )
                            .height(Length::FillPortion(3))
                            .width(Length::Fill),
                        );
                    }
                }
                layout = layout.push(layout_play_base);
            }
        }
        return layout.into();
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> button::Button<'a, UIMessage, iced::theme::Theme, iced::Renderer> {
        button(
            if key == String::from("backspace") || key == String::from("\u{e14a}") {
                text(format!("\u{e14a}")).font(MATERIAL_SYMBOLS_OUTLINED)
            } else {
                text(format!("{}", key))
            }
            .size(Pixels::from(28))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(text::Alignment::Center)
            .align_y(alignment::Vertical::Center)
            .style(|theme: &iced::theme::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = text::Style::default();
                style.color = Some(ex_palette.primary.base.text);
                style
            }),
        )
        .height(Length::Shrink)
        .width(Length::Shrink)
        .on_press(UIMessage::Home(HomeMessage::VSoftKeyBoardInput(format!(
            "{}",
            key
        ))))
        .style(rounded_primary_button)
    }

    pub fn title(&self) -> String {
        String::from("pmj_client_desktop")
    }

    pub fn theme(&self) -> iced::theme::Theme {
        self.theme.clone()
    }

    fn home_connect_server(&mut self) -> task::Task<UIMessage> {
        match connect(self.status_home.server_ip.clone()) {
            Ok((row_ws, resp)) => {
                trace!("resp={:?}", resp);
                let ws: sync::Arc<
                    sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>,
                > = sync::Arc::new(sync::RwLock::new(row_ws));
                self.ws = Some(ws.clone());
                debug!("Websocket 連線成功。");
                return task::Task::done(UIMessage::Home(HomeMessage::SendFirstMsg));
            }
            Err(e) => {
                warn!("ws connect error: {}", e);
                self.status_home.msgs.push(e.to_string());
                self.status_home.try_connecting_server = false;
            }
        }
        return task::Task::none();
    }
    fn home_send_first_msg(&mut self) -> task::Task<UIMessage> {
        //TODO: log::info!("正在嘗試傳送初連接訊息");
        trace!("正在嘗試傳送初連接訊息");
        let req_text = serde_json::to_string(&shared::ClientConnectRequestType {
            app_name: String::from("positive_mahjong"),
            client: String::from("pmj_client"),
        })
        .unwrap();
        match self.ws.clone() {
            Some(ws) => match ws.try_write() {
                Ok(mut guard) => match guard.send(Message::Text(req_text.into())) {
                    Ok(_) => {
                        debug!("已傳送初連結訊息，等待伺服器回應...");
                        return task::Task::done(UIMessage::Home(HomeMessage::ReadFirstMsgResp));
                    }
                    Err(e) => {
                        warn!("error: {}", e);
                        self.status_home.try_connecting_server = false;
                        return task::Task::none();
                    }
                },
                Err(e) => {
                    warn!("First msg: get guard error: {}", e);
                    return task::Task::done(UIMessage::Home(HomeMessage::SendFirstMsg));
                }
            },
            None => {
                return task::Task::done(UIMessage::Home(HomeMessage::ConnectServer));
            }
        }
    }

    fn home_read_first_msg_resp(&self) -> ProThread {
        let ws = self.ws.clone().unwrap();
        let handle = thread::spawn(move || {
            let msg_result: tungstenite::Result<Message>;
            'guard: {
                match ws.try_write() {
                    Ok(mut guard) => {
                        if !guard.can_read() {
                            warn!("guard.can_read() => false!");
                            return ThreadResult {
                                is_error: true,
                                result_read_first_msg_resp: None,
                                ..Default::default()
                            };
                        } else {
                            trace!("Reading first-msg resp...");
                            msg_result = guard.read();
                            debug!("First-msg read Finish");
                            break 'guard;
                        }
                    }
                    Err(e) => {
                        warn!("process_thread: {}", e);
                        return ThreadResult {
                            is_error: true,
                            result_read_first_msg_resp: None,
                            ..Default::default()
                        };
                    }
                }
            }
            match msg_result {
                Ok(raw_msg) => {
                    match raw_msg {
                        Message::Text(text) => {
                            let msg: shared::ServerConnectResponceType =
                                serde_json::from_str(&text).unwrap();
                            if msg.player_id.is_some() {
                                info!("成功取得玩家識別碼：{}", msg.player_id.unwrap());
                                return ThreadResult {
                                    is_error: false,
                                    result_read_first_msg_resp: Some(
                                        ThreadProcessResultReadFirstMsgResp {
                                            player_id: msg.player_id.unwrap(),
                                        },
                                    ),
                                    ..Default::default()
                                };
                            } else {
                                error!("error: msg.player_id is None");
                                return ThreadResult {
                                    is_error: true,
                                    result_read_first_msg_resp: None,
                                    ..Default::default()
                                };
                            }
                        }
                        _ => {
                            return ThreadResult {
                                is_error: true,
                                result_read_first_msg_resp: None,
                                ..Default::default()
                            };
                            /* TODO:BIN-MsgPack */
                        }
                    }
                }
                Err(e) => {
                    error!("raw_msg => Err: {}", e);
                    return ThreadResult {
                        is_error: true,
                        result_read_first_msg_resp: None,
                        ..Default::default()
                    };
                }
            }
        });
        ProThread {
            handle: handle,
            start_time: std::time::Instant::now(),
            process_type: ThreadProcessTypes::ReadFirstMsgResp,
        }
    }

    fn play_base_read_websocket(&self) -> ProThread {
        let ws = self.ws.clone().unwrap();
        let handle = thread::spawn(move || match ws.try_write() {
            Ok(mut guard) => match guard.read() {
                Ok(msg) => match msg {
                    Message::Text(t) => {
                        return ThreadResult {
                            is_error: false,
                            result_play_base_read_websocket: Some(t.to_string()),
                            ..Default::default()
                        };
                    }
                    _ => {
                        error!("err msg type!");
                        ThreadResult {
                            is_error: true,
                            ..Default::default()
                        }
                    }
                },
                Err(e) => {
                    error!("err:{:?}", e);
                    return ThreadResult {
                        is_error: true,
                        ..Default::default()
                    };
                }
            },
            Err(e) => {
                error!("err:{:?}", e);
                return ThreadResult {
                    is_error: true,
                    ..Default::default()
                };
            }
        });
        ProThread {
            handle: handle,
            start_time: std::time::Instant::now(),
            process_type: ThreadProcessTypes::PlayBaseReadWebsocket,
        }
    }
}

fn transparent_button(t: &iced::Theme, s: button::Status) -> button::Style {
    let p = t.extended_palette();
    let mut style = button::Style::default();
    style.border = Border {
        color: p.background.strong.color,
        width: 2.0,
        radius: iced::border::radius(10),
    };
    style.text_color = p.primary.base.text;
    match s {
        button::Status::Active => {
            style.background = None;
        }
        button::Status::Hovered => {
            style.background = Some(iced::Background::Color(iced::Color::from_rgba(
                1.0, 1.0, 1.0, 0.6,
            )));
        }
        button::Status::Disabled => {
            style.background = Some(iced::Background::Color(p.background.weak.color));
        }
        button::Status::Pressed => {
            style.text_color = p.secondary.base.color;
        }
    }
    style
}

fn rounded_primary_button(t: &iced::Theme, s: button::Status) -> button::Style {
    let p = t.extended_palette();
    let mut style = button::Style::default();
    style.background = Some(iced::Background::Color(p.primary.base.color));
    style.text_color = p.primary.base.text;
    let mut border = iced::Border::default().rounded(14).width(2);
    match s {
        button::Status::Active => {
            border = border.color(iced::Color::TRANSPARENT);
        }
        button::Status::Disabled => {
            style.background = Some(iced::Background::Color(p.background.weak.color));
        }
        button::Status::Hovered => {
            border = border.color(p.primary.strong.color);
        }
        button::Status::Pressed => {
            style.text_color = p.secondary.base.color;
        }
    }
    style.border = border;
    style
}

fn write_reply(
    text: String,
    websocket: sync::Arc<sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>>,
) -> Result<(), tungstenite::error::Error> {
    trace!(type= "enter_func", arg_text = ?text);
    let reply: Message = Message::Text(text.into());
    let write_result: tungstenite::Result<()>;
    loop {
        match websocket.try_write() {
            Ok(mut guard) => {
                write_result = guard.write(reply.clone());
                let _ = guard.flush();
                drop(guard);
                break;
            }
            Err(e) => {
                warn!("ws.try_write() Err: {}", e);
            }
        };
        thread::sleep(time::Duration::from_millis(500));
    }
    match write_result {
        Ok(_) => {
            info!("成功回覆。")
        }
        Err(_) => {
            warn!("回覆失敗！")
        }
    }
    write_result
}
