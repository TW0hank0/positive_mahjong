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

//! Base玩法的GUI

use std::{
    sync::{self, Arc, RwLock},
    thread,
};

use iced::{
    self, Border,
    widget::{self, Column, Row, button, container, scrollable, space, text},
};
use image;
use local_ip_address;
use tracing::{debug, error, info, trace, warn};

use pmj_gamemodes::base;
use pmj_shared::shared::{FONT_NOTO_SANS_REG_BYTES, ICON_PNG_BYTES, PROJECT_NAME};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn gui_init() -> Option<iced::window::Icon> {
    let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
    //
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let icon = iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    icon
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
        default_text_size: iced::Pixels::from(24),
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

#[derive(Debug, Clone, Copy)]
enum GUIMessages {
    StartGame,
    FetchPlayerInfo,
    CopyIp,
}

#[derive(Debug)]
struct ServerGUI {
    backend: Arc<RwLock<base::mode::PositiveMahjong>>,
    local_ipv4_address: std::net::IpAddr,
    local_ipv6_address: std::net::IpAddr,
    msg: String,
    is_start: bool,
    players: Vec<base::shared::PMJPlayer>,
}

impl ServerGUI {
    fn new() -> Self {
        let ipv4_address = local_ip_address::local_ip().unwrap();
        let ipv6_address = local_ip_address::local_ipv6().unwrap();
        info!("第四代網路地址：{}", ipv4_address.to_string());
        info!("第六代網路地址：{}", ipv6_address.to_string());
        info!("端口：{}", pmj_shared::shared::SERVER_PORT);
        let backend = base::mode::main_base(true).unwrap();
        Self {
            backend: backend,
            local_ipv4_address: ipv4_address,
            local_ipv6_address: ipv6_address,
            msg: String::new(),
            is_start: false,
            players: Vec::new(),
        }
    }

    fn update(&mut self, msg: GUIMessages) -> iced::Task<GUIMessages> {
        match msg {
            GUIMessages::StartGame => {
                match self.backend.try_read() {
                    Ok(backend) => {
                        self.players = backend.get_players_info();
                    }
                    Err(e) => {
                        warn!("FetchPlayerInfo error: {}", e);
                        return iced::task::Task::done(GUIMessages::StartGame);
                    }
                }
                if self.players.len() < 1 {
                    warn!("至少需要一位玩家！");
                    self.msg.push_str("至少需要一位玩家！");
                } else {
                    self.is_start = true;
                    match self.backend.try_read() {
                        Ok(backend) => {
                            self.players = backend.get_players_info();
                            drop(backend);
                        }
                        Err(e) => {
                            warn!("FetchPlayerInfo error: {}", e);
                        }
                    }
                    let thread_backend = sync::Arc::clone(&self.backend);
                    let _handle = thread::spawn(move || {
                        match thread_backend.try_write() {
                            Ok(mut guard) => {
                                guard.start_game();
                                info!("game finished.");
                            }
                            Err(e) => {
                                error!("Fail to start game: {}", e);
                            }
                        };
                    });
                }
            }
            GUIMessages::FetchPlayerInfo => match self.backend.try_read() {
                Ok(backend) => {
                    self.players = backend.get_players_info();
                }
                Err(e) => {
                    warn!("FetchPlayerInfo error: {}", e);
                    return iced::task::Task::done(GUIMessages::FetchPlayerInfo);
                }
            },
            GUIMessages::CopyIp => {
                // TODO: handle task
                return iced::clipboard::write::<GUIMessages>(self.local_ipv6_address.to_string());
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::widget::Column<'_, GUIMessages> {
        let mut layout: iced::widget::Column<'_, GUIMessages> = Column::new().spacing(30);
        {
            let mut ip_bar_layout = Column::new().spacing(30);
            ip_bar_layout = ip_bar_layout.push(
                text(format!("Ipv4: {}", self.local_ipv4_address))
                    .size(28)
                    .style(|theme: &iced::Theme| {
                        let ex_palette = theme.extended_palette();
                        let mut style = text::Style::default();
                        style.color = Some(ex_palette.secondary.base.text);
                        style
                    }),
            );
            ip_bar_layout = ip_bar_layout.spacing(40);
            ip_bar_layout = ip_bar_layout.push(
                Row::new()
                    .push(
                        text(format!("Ipv6: {}", self.local_ipv6_address))
                            .size(iced::Pixels::from(28))
                            .style(|theme: &iced::Theme| {
                                let ex_palette = theme.extended_palette();
                                let mut style = text::Style::default();
                                style.color = Some(ex_palette.secondary.base.text);
                                style
                            }),
                    )
                    .push(space().width(10))
                    .push(
                        button("複製")
                            .on_press(GUIMessages::CopyIp)
                            .style(transparent_button),
                    ),
            );
            let ip_bar_container = container(ip_bar_layout).style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = iced::widget::container::Style::default();
                style = style
                    .background(ex_palette.secondary.base.color)
                    .border(iced::border::Border::default().rounded(iced::border::radius(10.0)));
                style
            });
            layout = layout.push(ip_bar_container).spacing(80);
        }
        if !self.is_start {
            let start_button = button(text("開始").size(30))
                .on_press(GUIMessages::StartGame)
                .style(rounded_primary_button);
            layout = layout.push(start_button);
        } else {
            layout = layout.push(text("遊戲已開始").size(30).style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = text::Style::default();
                style.color = Some(ex_palette.background.strong.text);
                style
            }))
        }
        layout = layout.spacing(50);
        //
        layout = layout.push(
            button(text("重新整理").size(30))
                .on_press(GUIMessages::FetchPlayerInfo)
                .style(rounded_primary_button),
        );
        let mut player_info = Column::new();
        if self.players.len() > 0 {
            for player in self.players.iter() {
                let mut info_bar = Row::new();
                info_bar = info_bar
                    .push(
                        text(player.player_id)
                            .size(20)
                            .style(|theme: &iced::Theme| {
                                let ex_palette = theme.extended_palette();
                                let mut style = text::Style::default();
                                style.color = Some(ex_palette.primary.base.text);
                                style
                            }),
                    )
                    .spacing(50);
                info_bar = info_bar.push(text(player.player_ip_addr.to_string()).size(20).style(
                    |theme: &iced::Theme| {
                        let ex_palette = theme.extended_palette();
                        let mut style = text::Style::default();
                        style.color = Some(ex_palette.primary.weak.text);
                        style
                    },
                ));
                player_info = player_info.push(container(info_bar).style(|theme: &iced::Theme| {
                    let ex_palette = theme.extended_palette();
                    container::Style::default()
                        .border(iced::Border::default().rounded(8))
                        .background(ex_palette.primary.base.color)
                }));
            }
        } else {
            player_info = player_info.push(
                container(text("無人連線").size(22).style(|theme: &iced::Theme| {
                    let ex_palette = theme.extended_palette();
                    let mut style = text::Style::default();
                    style.color = Some(ex_palette.primary.base.text);
                    style
                }))
                .style(|theme: &iced::Theme| {
                    let ex_palette = theme.extended_palette();
                    container::Style::default()
                        .border(iced::Border::default().rounded(8))
                        .background(ex_palette.primary.base.color)
                }),
            );
        }
        layout = layout.push(
            container(scrollable(player_info)).style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                container::Style::default()
                    .border(iced::border::Border::default().rounded(12))
                    .background(ex_palette.background.strong.color)
            }),
        );
        layout = layout.push(
            container(scrollable(
                text(self.msg.clone())
                    .size(14)
                    .wrapping(text::Wrapping::WordOrGlyph),
            ))
            .style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                container::Style::default()
                    .background(ex_palette.background.weak.color)
                    .border(iced::border::Border::default().rounded(12))
            }),
        );
        //
        return layout;
    }

    pub fn title(&self) -> String {
        format!("{} - pmj_server::gui", PROJECT_NAME)
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNight
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
