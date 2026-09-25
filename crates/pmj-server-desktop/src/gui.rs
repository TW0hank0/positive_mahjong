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

//! V2Better玩法的GUI

use std::{
    net,
    sync::{Arc, RwLock},
    thread, time,
};

use iced::{
    self, Border, Length,
    widget::{Column, Row, button, container, rule, scrollable, space, text, text_input},
};
use tracing::{error, info, warn};

use pmj_gamemodes;
use pmj_shared::shared::{FONT_NOTO_SANS_REG_BYTES, ICON_PNG_BYTES, PROJECT_NAME};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn gui_init() -> Option<iced::window::Icon> {
    let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
    //
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();

    iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok()
}

pub fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        maximized: true,
        min_size: Some(iced::Size::new(720.0, 480.0)),
        icon: gui_init(),
        position: iced::window::Position::Centered,
        ..Default::default()
    };
    let app_settings = iced::Settings {
        id: Some(format!("{} - pmj_server::gui", PROJECT_NAME)),
        default_text_size: iced::Pixels::from(22),
        default_font: FONT_NOTO_SANS_REG,
        vsync: true,
        fonts: vec![std::borrow::Cow::from(FONT_NOTO_SANS_REG_BYTES)],
        ..Default::default()
    };
    iced::application(ServerGUI::new, ServerGUI::update, ServerGUI::view)
        .title(ServerGUI::title)
        .theme(ServerGUI::theme)
        .subscription(ServerGUI::subscription)
        .settings(app_settings)
        .window(window_settings)
        .run()
}

#[derive(Debug, Clone)]
enum GUIMessages {
    Home(HomeMsg),
    V2Better(V2BetterMsg),
    CopyToClipboard(String),
}

#[derive(Debug, Clone)]
pub enum V2BetterMsg {
    SendRoomMsg,
    TInputRoomMsgChange(String),
    StartGame,
    FetchPlayer,
}

#[derive(Debug, Clone)]
pub enum HomeMsg {
    StartServer,
}

#[derive(Debug)]
pub enum ServerScene {
    Home,
    V2BetterServer(V2BetterState),
}

#[derive(Debug)]
pub enum GameStatus {
    Room,
    InGame,
}

#[derive(Debug)]
pub struct V2BetterState {
    backend: Arc<RwLock<pmj_gamemodes::v2_better::mode::PositiveMahjong>>,
    game_status: GameStatus,
    tinput_room_msg: String,
    local_ipv4_address: net::IpAddr,
    local_ipv6_address: net::IpAddr,
    ip_port: u16,
    players: Vec<pmj_gamemodes::v2_better::shared::PMJPlayer>,
}

#[derive(Debug)]
struct ServerGUI {
    scene: ServerScene,
    msg: String,
    theme: iced::Theme,
}

impl ServerGUI {
    fn new() -> Self {
        Self {
            scene: ServerScene::Home,
            msg: String::new(),
            theme: iced::Theme::TokyoNight,
        }
    }

    fn update(&mut self, msg: GUIMessages) -> iced::Task<GUIMessages> {
        match msg {
            GUIMessages::Home(home_msg) => match self.scene {
                ServerScene::Home => match home_msg {
                    HomeMsg::StartServer => {
                        let backend = pmj_gamemodes::v2_better::mode::main_v2_better(true).unwrap();
                        let ipv4_address = local_ip_address::local_ip().unwrap();
                        let ipv6_address = local_ip_address::local_ipv6().unwrap();
                        let ip_port = pmj_shared::shared::SERVER_PORT;
                        info!("第四代網路地址：{}", ipv4_address.to_string());
                        info!("第六代網路地址：{}", ipv6_address.to_string());
                        info!("端口：{}", pmj_shared::shared::SERVER_PORT);
                        self.scene = ServerScene::V2BetterServer(V2BetterState {
                            backend: backend,
                            game_status: GameStatus::Room,
                            tinput_room_msg: String::new(),
                            local_ipv4_address: ipv4_address,
                            local_ipv6_address: ipv6_address,
                            ip_port: ip_port,
                            players: Vec::new(),
                        })
                    }
                },
                ServerScene::V2BetterServer(ref _v2_state) => {
                    warn!("update: warn scene");
                }
            },
            GUIMessages::V2Better(v2_msg) => match self.scene {
                ServerScene::Home => {
                    warn!("update: warn scene");
                }
                ServerScene::V2BetterServer(ref mut v2_state) => match v2_msg {
                    V2BetterMsg::FetchPlayer => match v2_state.backend.try_read() {
                        Ok(guard) => {
                            v2_state.players = guard.get_players_info();
                        }
                        Err(e) => {
                            warn!("update: {}", e);
                        }
                    },
                    V2BetterMsg::StartGame => match v2_state.backend.try_write() {
                        Ok(mut guard) => {
                            guard.start_game();
                            v2_state.game_status = GameStatus::InGame;
                        }
                        Err(e) => {
                            warn!("update: {}", e);
                        }
                    },
                    V2BetterMsg::TInputRoomMsgChange(room_msg_draft) => {
                        v2_state.tinput_room_msg = room_msg_draft;
                    }
                    V2BetterMsg::SendRoomMsg => {
                        let thread_backend = v2_state.backend.clone();
                        let msg = v2_state.tinput_room_msg.clone();
                        thread::spawn(move || {
                            loop {
                                match thread_backend.try_read() {
                                    Ok(guard) => {
                                        guard.say_root_msg(msg);
                                        break;
                                    }
                                    Err(e) => {
                                        warn!("update: {}", e);
                                        thread::sleep(time::Duration::from_millis(750));
                                    }
                                }
                            }
                        });
                        v2_state.tinput_room_msg.clear();
                    }
                },
            },
            GUIMessages::CopyToClipboard(content) => {
                // TODO: handle task
                return iced::clipboard::write(content);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, GUIMessages> {
        let mut layout = Vec::new();
        match self.scene {
            ServerScene::Home => {
                let mut home_layout = Vec::new();
                {
                    home_layout.push(
                        button(text("Start Server"))
                            .on_press(GUIMessages::Home(HomeMsg::StartServer))
                            .into(),
                    );
                }
                layout.push(Column::from_vec(home_layout).spacing(5).into());
            }
            ServerScene::V2BetterServer(ref v2_state) => {
                let mut v2_layout = Vec::new();
                {
                    let mut ip_bar = Vec::new();
                    ip_bar.push(
                        button(text(format!("Ipv4: {}", v2_state.local_ipv4_address)))
                            .on_press(GUIMessages::CopyToClipboard(
                                v2_state.local_ipv4_address.to_string(),
                            ))
                            .into(),
                    );
                    ip_bar.push(space().height(Length::from(4)).into());
                    ip_bar.push(rule::horizontal(iced::Pixels::from(1.5)).into());
                    ip_bar.push(space().height(Length::from(4)).into());
                    ip_bar.push(
                        button(text(format!("Ipv6: {}", v2_state.local_ipv6_address)))
                            .on_press(GUIMessages::CopyToClipboard(
                                v2_state.local_ipv6_address.to_string(),
                            ))
                            .into(),
                    );
                    v2_layout.push(
                        container(Column::from_vec(ip_bar))
                            .style(primary_outlined_container)
                            .padding(10)
                            .into(),
                    );
                }
                {
                    let mut msg_bar_layout = Vec::new();
                    msg_bar_layout.push(
                        text_input("say room msg as root", &v2_state.tinput_room_msg)
                            .on_input(|content| {
                                GUIMessages::V2Better(V2BetterMsg::TInputRoomMsgChange(content))
                            })
                            .on_submit(GUIMessages::V2Better(V2BetterMsg::SendRoomMsg))
                            .width(Length::Fill)
                            .into(),
                    );
                    msg_bar_layout.push(space().width(3).into());
                    msg_bar_layout.push(
                        button(text("Send"))
                            .on_press(GUIMessages::V2Better(V2BetterMsg::SendRoomMsg))
                            .width(Length::Shrink)
                            .into(),
                    );
                    v2_layout.push(
                        Row::from_vec(msg_bar_layout)
                            .spacing(3)
                            .width(Length::Fill)
                            .into(),
                    );
                }
                layout.push(Column::from_vec(v2_layout).spacing(5).into());
            }
        }
        Column::from_vec(layout).padding(3).into()
    }

    pub fn title(&self) -> String {
        format!("{} - pmj_server::gui", PROJECT_NAME)
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    pub fn subscription(&self) -> iced::Subscription<GUIMessages> {
        iced::Subscription::none()
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

pub fn primary_outlined_container(theme: &iced::Theme) -> container::Style {
    let p = theme.extended_palette();
    container::Style {
        border: Border {
            color: p.primary.base.color,
            width: 0.7,
            radius: iced::border::radius(8),
        },
        ..Default::default()
    }
}
