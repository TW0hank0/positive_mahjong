// SPDX-License-Identifier: AGPL-3.0-only
// 版權所有 (C) 2026 TW0hank0
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

use std;

use iced::{
    self, Border, Color, Element, Length, Pixels, alignment, task,
    widget::{self, Column, Row, button, container, scrollable, space, stack, text, text_input},
};
use tracing::{error, warn};

use pmj_desktop::{
    circular, easing,
    shared::{ButtonStyles, ContainerStyles},
};

use pmj_shared::shared::{
    self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES, PROJECT_NAME,
};

// pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");
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
    game_msgs: Vec<(u64, String)>,
    game_controller: Vec<pmj_client_core::ccore::PlayerCtrl>,
    current_turn: Option<u8>,
    ccore: pmj_client_core::ccore::ClientCore,
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
    GetCard,
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
        Self {
            server_url: String::from("ws://"),
            scene: ClientScenes::Home(HomeState {
                try_connecting_server: false,
                msgs: Vec::new(),
                connect_msg: None,
            }),
            theme: iced::theme::Theme::TokyoNight,
        }
    }
    pub fn update(&mut self, message: UIMessage) -> task::Task<UIMessage> {
        match message {
            UIMessage::CCoreProcessTask => match self.scene {
                ClientScenes::Home(ref _home_state) => {
                    warn!("update: warn scene");
                }
                ClientScenes::Play(ref mut play_state) => {
                    play_state.ccore.process_task();
                    play_state.game_controller = play_state.ccore.current_ctrl();
                    play_state.gm_state = play_state.ccore.game_state();
                    match play_state.gm_state {
                        pmj_client_core::ccore::GMState::HomePage => {
                            warn!("update: warn GMState");
                        }
                        pmj_client_core::ccore::GMState::V2Better(ref gms_v2) => {
                            play_state.current_turn = gms_v2.player_turn;
                            play_state.player_id = gms_v2.player_id;
                            if (!play_state.is_start)
                                && gms_v2.game_events.iter().any(|(_, event)| match event {
                                    pmj_client_core::ccore::V2BetterEvents::GameStart => true,
                                    _ => false,
                                })
                            {
                                play_state.is_start = true;
                            }
                            play_state.hand_cards = gms_v2.cards.clone();
                            play_state.game_msgs.clear();
                            for (msgid, gmsg) in gms_v2.game_events.clone().iter() {
                                play_state
                                    .game_msgs
                                    .push((msgid.clone(), format!("{:?}", gmsg)));
                            }
                        }
                    }
                    return iced::Task::done(UIMessage::CCoreProcessTask);
                }
            },
            UIMessage::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => match self.scene {
                    ClientScenes::Home(ref mut home_state) => {
                        if home_state.try_connecting_server {
                            home_state
                                .msgs
                                .push(String::from("已有正在嘗試連接的伺服器！"));
                        } else {
                            self.server_url = server_ip;
                        }
                    }
                    ClientScenes::Play(ref _play_state) => {
                        warn!("update: warn scene");
                    }
                },
                HomeMessage::VSoftKeyBoardInput(key) => match self.scene {
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
                },
                HomeMessage::ConnectServer => match self.scene {
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
                                Ok(mut ccore) => {
                                    ccore.process_task();
                                    let mut gm_state = ccore.game_state();
                                    loop {
                                        match gm_state.clone() {
                                            pmj_client_core::ccore::GMState::HomePage => {
                                                warn!("update: warn GMState");
                                                std::thread::sleep(
                                                    std::time::Duration::from_millis(500),
                                                );
                                                ccore.process_task();
                                                gm_state = ccore.game_state();
                                            }
                                            pmj_client_core::ccore::GMState::V2Better(gms_v2) => {
                                                self.scene = ClientScenes::Play(PlayState {
                                                    is_start: false,
                                                    hand_cards: Vec::new(),
                                                    game_msgs: Vec::with_capacity(20),
                                                    game_controller: Vec::new(),
                                                    current_turn: None,
                                                    ccore,
                                                    gm_state,
                                                    player_id: gms_v2.player_id,
                                                });
                                                break;
                                            }
                                        }
                                    }
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
                },
            },
            UIMessage::Play(play_base_message) => match play_base_message {
                PlayMsg::GetCard => match self.scene {
                    ClientScenes::Home(ref _home_state) => {
                        warn!("update: warn scene");
                    }
                    ClientScenes::Play(ref mut play_state) => {
                        play_state.ccore.player_game_action(
                            pmj_gamemodes::v2_better::shared::PlayerGameActions::GetCard,
                        );
                    }
                },
                PlayMsg::ThrowCard(card) => match self.scene {
                    ClientScenes::Home(ref _home_state) => {
                        warn!("update: warn scene");
                    }
                    ClientScenes::Play(ref mut play_state) => {
                        play_state.ccore.player_game_action(
                            pmj_gamemodes::v2_better::shared::PlayerGameActions::ThrowCard(card),
                        );
                    }
                },
            },
        }
        iced::task::Task::none()
    }

    pub fn view(&self) -> Element<'_, UIMessage, iced::Theme, iced::Renderer> {
        let mut layout: Vec<iced::Element<'_, UIMessage>> = Vec::new();
        match self.scene {
            ClientScenes::Home(ref home_state) => {
                let mut layout_home: Vec<iced::Element<'_, UIMessage>> = Vec::new();
                // 標題欄
                {
                    let mut title_bar = Vec::new();
                    title_bar.push(
                        text(shared::PROJECT_NAME.to_string())
                            .height(Length::Shrink)
                            .size(Pixels::from(26))
                            .into(),
                    );
                    title_bar.push(space().width(20).into());
                    title_bar.push(
                        text(format!("v{}", shared::PROJECT_VERSION))
                            .height(Length::Shrink)
                            .size(Pixels::from(22))
                            .into(),
                    );
                    layout_home.push(
                        Row::from_vec(title_bar)
                            .align_y(alignment::Vertical::Center)
                            .into(),
                    );
                }
                layout_home.push(space().height(5).into());
                // 伺服器地址輸入處理
                {
                    let mut server_ip_input_bar = Vec::new();
                    server_ip_input_bar.push(
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
                            })
                            .into(),
                    );
                    server_ip_input_bar.push(
                        button(text("連線").size(24))
                            .on_press(UIMessage::Home(HomeMessage::ConnectServer))
                            .style(ButtonStyles::PrimaryRounded.style())
                            .into(),
                    );
                    layout_home.push(
                        Row::from_vec(server_ip_input_bar)
                            .width(Length::Fill)
                            .spacing(5)
                            .into(),
                    );
                    layout_home.push(space().height(10).into());
                }
                // 虛擬鍵盤
                {
                    let mut vsoft_keyboard = Vec::new();
                    vsoft_keyboard.push(
                        (0..=9)
                            .fold(Row::new(), |layout, i| {
                                layout.push(self.home_create_vsoft_key(i.to_string()))
                            })
                            .into(),
                    );
                    /* for key in 0..=9 {
                        vsoft_keyboard.push(self.home_create_vsoft_key(format!("{}", key)).into());
                    } */
                    vsoft_keyboard.push(
                        ALPHABET
                            .iter()
                            .fold(Row::new(), |layout, i| {
                                layout.push(self.home_create_vsoft_key(i.to_string()))
                            })
                            .into(),
                    );
                    /* for key in ALPHABET {
                        vsoft_keyboard.push(
                            self.home_create_vsoft_key(format!("{}", key).to_lowercase())
                                .into(),
                        );
                    } */
                    vsoft_keyboard.push(
                        [":", "[", "]", ".", "/", "backspace"]
                            .iter()
                            .fold(Row::new(), |layout, i| {
                                layout.push(self.home_create_vsoft_key(i.to_string()))
                            })
                            .into(),
                    );
                    /* for key in [":", "[", "]", ".", "/", "backspace"] {
                        vsoft_keyboard.push(self.home_create_vsoft_key(key.to_string()).into());
                    } */
                    layout_home.push(
                        Column::from_vec(vsoft_keyboard)
                            .spacing(4)
                            .align_x(alignment::Horizontal::Center)
                            .into(),
                    );
                    layout_home.push(space().height(20).into());
                }
                // 訊息顯示
                {
                    let mut msg_area = Vec::new();
                    let mut msg_number: u64 = 1;
                    for msg in home_state.msgs.iter() {
                        let ex_palette = self.theme.extended_palette();
                        let mut msg_row = Vec::new();
                        msg_row.push(
                            text(msg_number.to_string())
                                .size(20)
                                .style(move |_theme| text::Style {
                                    color: Some(ex_palette.secondary.strong.text),
                                })
                                .into(),
                        );
                        msg_row.push(
                            text(msg)
                                .size(20)
                                .style(move |_theme| text::Style {
                                    color: Some(ex_palette.secondary.base.text),
                                })
                                .into(),
                        );
                        msg_area.push(Row::from_vec(msg_row).spacing(3).into());
                        msg_number += 1;
                    }
                    layout_home.push(
                        container(Column::from_vec(msg_area))
                            .style(|theme: &iced::theme::Theme| {
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
                            })
                            .into(),
                    );
                }
                layout_home.push(space().height(5).into());
                //
                if home_state.try_connecting_server {
                    let mut content_column = Vec::new();
                    content_column.push(
                        circular::Circular::new()
                            .easing(&easing::STANDARD)
                            .size(54.0)
                            .into(),
                    );
                    content_column.push(
                        text("連線中...")
                            .size(28)
                            .style(move |theme: &iced::theme::Theme| {
                                let ex_palette = theme.extended_palette();
                                text::Style {
                                    color: Some(ex_palette.secondary.base.text),
                                }
                            })
                            .align_x(alignment::Horizontal::Center)
                            .into(),
                    );
                    content_column.push(
                        text(
                            home_state
                                .connect_msg
                                .clone()
                                .unwrap_or(String::from("None")),
                        )
                        .style(|_t: &iced::Theme| text::Style {
                            color: Some(iced::Color::from_rgb8(56, 56, 56)),
                        })
                        .size(22)
                        .into(),
                    );
                    let content = container(
                        container(Column::from_vec(content_column).padding(10).spacing(3)).style(
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
                        ),
                    )
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
                    layout.push(
                        stack([
                            widget::opaque(scrollable(
                                Column::from_vec(layout_home)
                                    .width(Length::Fill)
                                    .padding(10),
                            ))
                            .into(),
                            content.into(),
                        ])
                        .into(),
                    );
                } else {
                    layout.push(
                        scrollable(
                            Column::from_vec(layout_home)
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .padding(10),
                        )
                        .into(),
                    );
                }
            }
            ClientScenes::Play(ref play_state) => {
                let mut layout_play: Vec<iced::Element<'_, UIMessage>> = Vec::new();
                {
                    let mut info_bar = Vec::new();
                    info_bar.push(text(format!("伺服器地址：{}", self.server_url.clone())).into());
                    info_bar.push(space().width(15).into());
                    info_bar.push(
                        text(format!(
                            "玩家識別碼：{}",
                            (match play_state.gm_state.clone() {
                                pmj_client_core::ccore::GMState::HomePage => {
                                    warn!("view: 資料錯誤");
                                    "資料錯誤".to_string()
                                }
                                pmj_client_core::ccore::GMState::V2Better(gms_v2) => {
                                    gms_v2.player_id.to_string()
                                }
                            })
                        ))
                        .into(),
                    );
                    if play_state.is_start {
                        info_bar.push(space().width(15).into());
                        info_bar.push(
                            text(format!(
                                "目前回合：{}",
                                match play_state.current_turn {
                                    Some(turn) => {
                                        turn.to_string()
                                    }
                                    None => {
                                        warn!("view: play_state.current_turn => None");
                                        String::from("None")
                                    }
                                }
                            ))
                            .into(),
                        );
                    }
                    layout_play.push(
                        Row::from_vec(info_bar)
                            .width(Length::Fill)
                            .padding(8)
                            .into(),
                    );
                }
                // ctr_bar
                {
                    let mut ctr_bar: Vec<iced::Element<'_, UIMessage>> = Vec::new();
                    let mut gmsg_bar = Vec::new();
                    if play_state.is_start {
                        for (gmsg_id, gmsg) in play_state.game_msgs.iter() {
                            gmsg_bar.push(
                                container(
                                    Row::new()
                                        .push(text(gmsg_id.to_string()).size(17).style(
                                            |t: &iced::Theme| {
                                                let p = t.extended_palette();
                                                text::Style {
                                                    color: Some(p.primary.base.color),
                                                }
                                            },
                                        ))
                                        .push(space().width(15))
                                        .push(text(gmsg.clone()).size(16)),
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
                                })
                                .into(),
                            );
                            gmsg_bar.push(space().height(10).into());
                        }
                        ctr_bar.push(
                            scrollable(
                                Column::from_vec(gmsg_bar)
                                    .height(Length::Fill)
                                    .width(Length::FillPortion(3)),
                            )
                            .direction(scrollable::Direction::Vertical(
                                scrollable::Scrollbar::new()
                                    .margin(3)
                                    .spacing(5)
                                    .width(3)
                                    .scroller_width(6),
                            ))
                            .into(),
                        );
                    } else {
                        ctr_bar.push(space().width(Length::FillPortion(3)).into());
                    }
                    let mut rmsg_bar = Vec::new();
                    match play_state.gm_state {
                        pmj_client_core::ccore::GMState::HomePage => {
                            warn!("view: warn GMState");
                        }
                        pmj_client_core::ccore::GMState::V2Better(ref gms_v2) => {
                            for (rmsg_num, rmsg) in gms_v2.room_msgs.iter() {
                                let mut row_msg = Vec::new();
                                row_msg.push(
                                    text(rmsg_num)
                                        .size(18)
                                        .style(|t: &iced::Theme| {
                                            let p = t.extended_palette();
                                            text::Style {
                                                color: Some(p.primary.base.color),
                                            }
                                        })
                                        .into(),
                                );
                                row_msg.push(space().width(10).into());
                                let (said_player, said_text) = match rmsg {
                                    pmj_gamemodes::v2_better::shared::ServerRoomMsg::PlayerSay(
                                        said_player,
                                        said_text,
                                    ) => (format!("玩家@{}", said_player), said_text),
                                    pmj_gamemodes::v2_better::shared::ServerRoomMsg::RootSay(
                                        said_text,
                                    ) => (String::from("Root"), said_text),
                                };
                                row_msg.push(text(said_player).size(16).into());
                                row_msg.push(
                                    text("\u{e5c8}")
                                        .font(MATERIAL_SYMBOLS_OUTLINED)
                                        .size(14)
                                        .align_y(alignment::Vertical::Center)
                                        .into(),
                                );
                                row_msg.push(text(said_text).size(16).into());
                                rmsg_bar.push(Row::from_vec(row_msg).into());
                                rmsg_bar.push(space().height(10).into());
                            }
                        }
                    }
                    ctr_bar.push(
                        scrollable(
                            Column::from_vec(rmsg_bar)
                                .height(Length::Fill)
                                .width(Length::FillPortion(2)),
                        )
                        .direction(scrollable::Direction::Vertical(
                            scrollable::Scrollbar::new()
                                .margin(3)
                                .spacing(5)
                                .width(3)
                                .scroller_width(6),
                        ))
                        .into(),
                    );
                    layout_play.push(
                        Row::from_vec(ctr_bar)
                            .width(Length::Fill)
                            .height(Length::FillPortion(2))
                            .into(),
                    );
                }
                if !play_state.is_start {
                    let status_bar = Column::new().width(Length::Fill).push(
                        text("等待遊戲開始")
                            .size(30)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    );
                    layout_play.push(status_bar.into());
                } else {
                    let mut controller_bar = Vec::new();
                    {
                        // pga
                        let mut pga_bar = Vec::new();
                        if play_state
                            .game_controller
                            .contains(&pmj_client_core::ccore::PlayerCtrl::GetCard)
                        {
                            pga_bar.push(
                                button(text("GetCard"))
                                    .on_press(UIMessage::Play(PlayMsg::GetCard))
                                    .into(),
                            );
                        }
                        controller_bar.push(Row::from_vec(pga_bar).width(Length::Fill).into());
                    }
                    // 卡牌
                    {
                        let mut card_bar_elements: Vec<iced::Element<'_, UIMessage>> = Vec::new();
                        for card in play_state.hand_cards.iter() {
                            let card_element = Column::new()
                                .padding(5)
                                .width(120)
                                .height(160)
                                .push(text(card.to_string()).size(18))
                                .push(
                                    text(format!("第 {} 張", card.card_id.clone()))
                                        .height(Length::Fill)
                                        .align_y(alignment::Vertical::Bottom)
                                        .size(15)
                                        .align_x(alignment::Horizontal::Right),
                                );
                            if play_state
                                .game_controller
                                .contains(&pmj_client_core::ccore::PlayerCtrl::ThrowCard)
                            {
                                card_bar_elements.push(
                                    button(card_element)
                                        .on_press(UIMessage::Play(PlayMsg::ThrowCard(card.clone())))
                                        .style(ButtonStyles::PrimaryOutlined.style())
                                        .into(),
                                );
                            } else {
                                card_bar_elements.push(
                                    container(card_element)
                                        .style(ContainerStyles::BackgroundOutlined.style())
                                        .into(),
                                );
                            }
                        }
                        controller_bar.push(
                            Row::new()
                                .extend(card_bar_elements)
                                .spacing(7)
                                .padding(5)
                                .into(),
                        );
                        //layout_play.push(space().height(Length::Fill).into());
                        layout_play.push(
                            scrollable(
                                container(
                                    Column::from_vec(controller_bar)
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .spacing(7),
                                )
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
                                .height(Length::Shrink),
                            )
                            .direction(scrollable::Direction::Horizontal(
                                scrollable::Scrollbar::new()
                                    .margin(3)
                                    .spacing(5)
                                    .width(3)
                                    .scroller_width(6),
                            ))
                            .height(Length::Shrink)
                            .width(Length::Fill)
                            .into(),
                        );
                    }
                }
                layout.push(
                    Column::from_vec(layout_play)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into(),
                );
            }
        }
        Column::from_vec(layout)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(10)
            .into()
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> button::Button<'a, UIMessage, iced::theme::Theme, iced::Renderer> {
        button(
            if key == "backspace" || key == "\u{e14a}" {
                text("\u{e14a}").font(MATERIAL_SYMBOLS_OUTLINED)
            } else {
                text(key.clone())
            }
            .size(Pixels::from(28))
            .align_x(text::Alignment::Center)
            .align_y(alignment::Vertical::Center),
        )
        .height(Length::Shrink)
        .width(Length::Shrink)
        .on_press(UIMessage::Home(HomeMessage::VSoftKeyBoardInput(key)))
        .style(ButtonStyles::PrimaryRounded.style())
    }

    pub fn title(&self) -> String {
        format!("{} - pmj_client_desktop", PROJECT_NAME)
    }

    pub fn theme(&self) -> iced::theme::Theme {
        self.theme.clone()
    }
}
