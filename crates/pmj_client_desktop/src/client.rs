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

use std::{self, net::TcpStream};

use iced::{
    self, Border, Color, Element, Length, Pixels, alignment, task,
    widget::{
        self, Column, Grid, Row, button, container, grid, scrollable, space, stack, text,
        text_input,
    },
};
use tracing::{error, warn};
use tungstenite::WebSocket;

use crate::{circular, easing};

use pmj_shared::shared::{
    self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES, PROJECT_NAME,
};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");
pub const MATERIAL_SYMBOLS_OUTLINED: iced::font::Font =
    iced::font::Font::with_name("Material Symbols Outlined");

#[derive(Debug)]
pub struct Client {
    server_url: String,
    scene: ClientScenes,
    theme: iced::theme::Theme,
}

#[derive(Debug)]
pub enum ClientScenes {
    Home(HomeState),
    Play(PlayState),
}

#[derive(Debug, PartialEq, Eq)]
pub struct HomeState {
    try_connecting_server: bool,
    msgs: Vec<String>,
    connect_msg: Option<String>,
}

#[derive(Debug)]
pub struct PlayState {
    is_start: bool,
    player_id: u8,
    hand_cards: Vec<pmj_gamemodes::v2_better::shared::PMJCard>,
    game_msgs: Vec<String>,
    game_controller: pmj_client_core::ccore::PlayerCtrl,
    current_turn: Option<u8>,
    ccore:pmj_client_core::ccore::ClientCore,
    gm_state: pmj_client_core::ccore::GMState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIMessage {
    Home(HomeMessage),
    Play(PlayMsg),
    CCoreProcessTask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
    VSoftKeyBoardInput(String),
    ConnectServer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayMsg {
    ThrowCard(pmj_gamemodes::v2_better::shared::PMJCard),
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Client {
    pub fn new() -> Self {
        let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
        let _ = iced::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
        Self { server_url: String::new(), scene: ClientScenes::Home(HomeState { try_connecting_server: false, msgs: Vec::new(), connect_msg: None }), theme: iced::theme::Theme::TokyoNight }
    }
    pub fn update(&mut self, message: UIMessage) -> task::Task<UIMessage> {
        match message {
            UIMessage::CCoreProcessTask =>  {
                match self.scene {
                    ClientScenes::Home(ref _home_state) => {
                        warn!("update: warn scene");
                    }
                    ClientScenes::Play(ref mut play_state) => {
                        play_state.ccore.process_task();
                        play_state.game_controller=play_state.ccore.current_ctrl();
                        play_state.gm_state = play_state.ccore.game_state();
                        match play_state.gm_state {
                            pmj_client_core::ccore::GMState::HomePage => {
                                warn!("update: warn GMState");
                            }
                            pmj_client_core::ccore::GMState::V2Better(ref gms_v2) => {
                                play_state.current_turn = gms_v2.player_turn;
                                play_state.player_id = gms_v2.player_id;
                            }
                        }
                        return iced::Task::done(UIMessage::CCoreProcessTask);
                    }
                }
            },
            UIMessage::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => {
                    match self.scene {
                        ClientScenes::Home(ref mut home_state) => {
                            if home_state.try_connecting_server {
                                home_state.msgs.push(String::from("已有正在嘗試連接的伺服器！"));
                            } else {
                                self.server_url = server_ip;
                            }
                        }
                        ClientScenes::Play(ref _play_state) => {
                            warn!("update: warn scene");
                        }
                    }
                }
                HomeMessage::VSoftKeyBoardInput(key) => {
                    match self.scene {
                        ClientScenes::Home(ref mut home_state) => {
                            if home_state.try_connecting_server {
                                let msg = String::from("已有正在嘗試連接的伺服器！");
                                home_state.msgs.push(msg.clone());
                                warn!("update: {}", msg);
                            } else {
                                if key == "backspace" || key == "\u{e14a}" {
                                    self.server_url.pop();
                                } else {
                                    self.server_url.push_str(&key);
                                }
                            }
                        }
                        ClientScenes::Play(ref _play_state) => {
                            warn!("update: warn scene");
                        }
                    }
                }
                HomeMessage::ConnectServer => {
                    match self.scene {
                        ClientScenes::Home(ref mut home_state) => {
                            if self.server_url.is_empty() {
                                let msg = String::from("未輸入伺服器地址！");
                                home_state.msgs.push(msg.clone());
                                warn!("update: {}", msg);
                            } else if home_state.try_connecting_server {
                                let msg = String::from("已有正在嘗試連接的伺服器！");
                                home_state.msgs.push(msg.clone());
                                warn!("update: {}", msg);
                            } else {
                                    home_state.try_connecting_server = true;
                                    match pmj_client_core::ccore::ClientCore::connect(
                                        self.server_url.clone(),
                                    ) {
                                        Ok(ccore) => {
                                            let gm_state = ccore.game_state();
                                            loop{match gm_state.clone() {
                                                pmj_client_core::ccore::GMState::HomePage => {
                                                    warn!("update: warn GMState");
                                                    std::thread::sleep(std::time::Duration::from_millis(500));
                                                }
                                                pmj_client_core::ccore::GMState::V2Better(gms_v2) => {
                                                    self.scene = ClientScenes::Play(PlayState { is_start: false, hand_cards: Vec::new()
                                                        , game_msgs: Vec::new(), game_controller: pmj_client_core::ccore::PlayerCtrl::NoCtrl, current_turn: None, ccore, gm_state, player_id:gms_v2.player_id });
                                                    break;
                                                }
                                            }}
                                            return task::Task::done(UIMessage::CCoreProcessTask);
                                        }
                                        Err(e) => {
                                            error!("update: {}", e);
                                            home_state.msgs.push(format!("update: {}", e));
                                        }
                                    }

                            }
                        }
                        ClientScenes::Play(ref _play_state) => {
                            warn!("update: warn scene");
                        }
                    }
                }
            },
            UIMessage::Play(play_base_message) => match play_base_message {
                PlayMsg::ThrowCard(card) => {
                    match self.scene {
                        ClientScenes::Home(ref _home_state) => {
                            warn!("update: warn scene");
                        }
                        ClientScenes::Play(ref mut play_state) => {
                            play_state.ccore.throw_card(card);
                        }
                    }
                }
            },
        }
        iced::task::Task::none()
    }

    pub fn view(&self) -> Element<'_, UIMessage, iced::Theme, iced::Renderer> {
        let mut layout: Column<'_, UIMessage, iced::Theme, iced::Renderer> = Column::new()
            .align_x(alignment::Horizontal::Left)
            .padding(10);
        //
        match self.scene {
            ClientScenes::Home(ref home_state) => {
                let mut layout_home = Column::new();
                // 標題欄
                {
                    let mut title_bar = Row::new().align_y(alignment::Vertical::Center);
                    title_bar = title_bar.push(
                        text(shared::PROJECT_NAME.to_string())
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
                            text_input("輸入伺服器地址...", &self.server_url)
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
                            .push(self.home_create_vsoft_key(key.to_string()))
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
                    for msg in home_state.msgs.iter() {
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
                if home_state.try_connecting_server {
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
                            home_state
                                .connect_msg
                                .clone()
                                .unwrap_or(String::from("None")),
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
            ClientScenes::Play(ref play_state) => {
                let mut layout_play = Column::new();
                {
                    let mut info_bar = Row::new().padding(iced::Padding::new(8.0));
                    info_bar = info_bar.push(text(format!(
                        "伺服器地址：{}",
                        self.server_url.clone(),
                    )));
                    info_bar = info_bar.push(space().width(Length::from(14)));
                    info_bar =
                        info_bar.push(text(format!("玩家識別碼：{}", (match play_state.gm_state.clone() {
                            pmj_client_core::ccore::GMState::HomePage => { warn!("資料錯誤");"資料錯誤".to_string() }
                            pmj_client_core::ccore::GMState::V2Better(gms_v2) => { gms_v2.player_id.to_string()}
                        }))));
                    if play_state.is_start {
                        info_bar = info_bar.push(space().width(10)).push(text(format!(
                            "目前回合：{}",
                            match play_state.current_turn {
                                Some(turn) => {turn.to_string()}
                                None => {
                                    warn!("view: play_state.current_turn => None");
                                    String::from("None")
                                }
                            }
                        )));
                    }
                    layout_play = layout_play.push(info_bar)
                }
                if !play_state.is_start {
                    let mut status_bar = Column::new();
                    status_bar = status_bar.push(
                        text("等待遊戲開始")
                            .size(30)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    );
                    layout_play = layout_play.push(status_bar);
                } else {
                    {
                        let mut ctr_bar = Row::new().height(Length::FillPortion(2));
                        let mut msg_bar = Column::new().width(Length::FillPortion(3));
                        let mut msg_num: u16 = 1;
                        for msg in play_state.game_msgs.iter() {
                            msg_bar = msg_bar
                                .push(
                                    container(
                                        Row::new()
                                            .push(text(msg_num.to_string()).size(17).style(
                                                |t: &iced::Theme| {
                                                    let p = t.extended_palette();
                                                    text::Style {
                                                        color: Some(p.primary.base.color),
                                                    }
                                                },
                                            ))
                                            .push(space().width(15))
                                            .push(text(msg.clone()).size(16)),
                                    )
                                    .style(
                                        |t: &iced::Theme| {
                                            let p = t.extended_palette();
                                            let mut style = container::Style::default();
                                            style.border.radius = iced::border::Radius::new(10);
                                            style.border.width = 1.2;
                                            style.border.color = p.background.weak.color;
                                            style.text_color = Some(p.background.base.text);
                                            style.background = Some(iced::Background::Color(
                                                iced::Color::TRANSPARENT,
                                            ));
                                            style
                                        },
                                    ),
                                )
                                .push(space().height(10));
                            msg_num += 1;
                        }
                        ctr_bar = ctr_bar.push(scrollable(msg_bar));
                        let mut card_bar = Column::new().width(Length::FillPortion(2));
                        for card in play_state.hand_cards.iter() {
                            card_bar = card_bar.push(space().height(5)).push(
                                container(
                                    Row::new()
                                        .padding(10)
                                        .width(Length::Fill)
                                        .height(40)
                                        .push(text(card.to_string()).size(24))
                                        .push(
                                            text(format!("第 {} 張", card.card_id.clone()))
                                                .width(Length::Fill)
                                                .size(15)
                                                .align_x(alignment::Horizontal::Right),
                                        ),
                                )
                                .style(|t: &iced::Theme| {
                                    let p = t.extended_palette();
                                    let mut style = container::Style::default();
                                    style.border.radius = iced::border::Radius::new(10);
                                    style.border.width = 1.2;
                                    style.border.color = p.background.weak.color;
                                    style.text_color = Some(p.background.base.text);
                                    style.background =
                                        Some(iced::Background::Color(iced::Color::TRANSPARENT));
                                    style
                                }),
                            );
                        }
                        ctr_bar = ctr_bar.push(scrollable(card_bar));
                        layout_play = layout_play.push(ctr_bar);
                    }
                    // 玩家操作
                    {
                        let mut controller_bar = Column::new();
                        match play_state.game_controller {
                            pmj_client_core::ccore::PlayerCtrl::NoCtrl => {}
                            pmj_client_core::ccore::PlayerCtrl::ThrowCard => {
                                controller_bar = controller_bar.push(text("選擇一張你要丟的牌"));
                                let mut card_bar_elements: Vec<iced::Element<'_, UIMessage>> =
                                    Vec::new();
                                for card in play_state.hand_cards.iter() {
                                    card_bar_elements.push(
                                        button(
                                            Column::new()
                                                .width(120)
                                                .height(160)
                                                .push(text(card.to_string()).size(18))
                                                .push(
                                                    text(format!("第 {} 張", card.card_id.clone()))
                                                        .height(Length::Fill)
                                                        .align_y(alignment::Vertical::Bottom)
                                                        .size(15)
                                                        .align_x(alignment::Horizontal::Right),
                                                ),
                                        )
                                        .on_press(UIMessage::Play(
                                            PlayMsg::ThrowCard(card.clone()),
                                        ))
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
                        layout_play = layout_play.push(
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
                layout = layout.push(layout_play);
            }
        }
        layout.into()
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> button::Button<'a, UIMessage, iced::theme::Theme, iced::Renderer> {
        button(
            if key == "backspace" || key == "\u{e14a}" {
                text("\u{e14a}".to_string()).font(MATERIAL_SYMBOLS_OUTLINED)
            } else {
                text(key.to_string())
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
        .on_press(UIMessage::Home(HomeMessage::VSoftKeyBoardInput(
            key.to_string(),
        )))
        .style(rounded_primary_button)
    }

    pub fn title(&self) -> String {
        format!("{} - pmj_client_desktop", PROJECT_NAME)
    }

    pub fn theme(&self) -> iced::theme::Theme {
        self.theme.clone()
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
