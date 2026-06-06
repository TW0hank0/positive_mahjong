// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://github.com/TW0hank0/positive_mahjong
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
    self, Border, Theme,
    widget::{self, Column, Row, button, container, scrollable, text},
};

use image;
use local_ip_address;

use crate::base;

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
    let icon = gui_init();
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
    window_settings.position = iced::window::Position::Centered;
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(PROJECT_NAME));
    app_settings.default_text_size = iced::Pixels::from(24);
    app_settings.default_font = FONT_NOTO_SANS_REG;
    iced::application(ServerGUI::new, ServerGUI::update, ServerGUI::view)
        .title(ServerGUI::title)
        .theme(ServerGUI::theme)
        .subscription(ServerGUI::subscription)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum GUIMessages {
    StartGame,
    FetchPlayerInfo,
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
        println!("ipv4: {}", ipv4_address.to_string());
        println!("ipv6: {}", ipv6_address.to_string());
        println!("port: {}", pmj_shared::shared::SERVER_PORT);
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
                self.is_start = true;
                let thread_backend = sync::Arc::clone(&self.backend);
                match thread_backend.try_write() {
                    Ok(mut guard) => {
                        guard.start_game();
                    }
                    Err(e) => {
                        eprintln!("Fail to start game:{}", e);
                        return iced::task::Task::done(GUIMessages::StartGame);
                        //TODO: error handle
                    }
                };
                return iced::task::Task::done(GUIMessages::FetchPlayerInfo);
            }
            GUIMessages::FetchPlayerInfo => match self.backend.try_read() {
                Ok(backend) => {
                    self.players = backend.get_players_info();
                }
                Err(e) => {
                    eprintln!("FetchPlayerInfo error: {}", e);
                    return iced::task::Task::done(GUIMessages::FetchPlayerInfo);
                }
            },
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::widget::Column<'_, GUIMessages> {
        let mut layout: iced::widget::Column<'_, GUIMessages> = Column::new().spacing(30);
        //
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
        ip_bar_layout = ip_bar_layout
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
            .spacing(20);
        let ip_bar_container = container(ip_bar_layout).style(|theme: &iced::Theme| {
            let ex_palette = theme.extended_palette();
            let mut style = iced::widget::container::Style::default();
            style = style
                .background(ex_palette.secondary.base.color)
                .border(iced::border::Border::default().rounded(iced::border::radius(10.0)));
            style
        });
        layout = layout.push(ip_bar_container).spacing(80);
        //
        if !self.is_start {
            let start_button = widget::button(text("開始").size(30))
                .on_press(GUIMessages::StartGame)
                .style(|theme: &Theme, status: button::Status| {
                    let ex_palette = theme.extended_palette();
                    let mut style = button::Style::default();
                    match status {
                        button::Status::Active => {
                            style = style.with_background(ex_palette.primary.base.color);
                            style.text_color = ex_palette.primary.base.text;
                        }
                        button::Status::Disabled => {
                            style = style.with_background(ex_palette.background.weak.color);
                            style.text_color = ex_palette.background.weak.text;
                        }
                        button::Status::Hovered => {
                            style = style.with_background(ex_palette.primary.weak.color);
                            style.text_color = ex_palette.primary.weak.text;
                        }
                        button::Status::Pressed => {
                            style = style.with_background(ex_palette.primary.strong.color);
                            style.text_color = ex_palette.primary.strong.text;
                        }
                    }
                    style.border = Border::default()
                        .width(5)
                        .color(ex_palette.primary.strong.color)
                        .rounded(10);
                    style
                });
            layout = layout.push(start_button);
        } else {
            layout = layout.push(text("遊戲已開始！").size(30).style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = text::Style::default();
                style.color = Some(ex_palette.background.strong.text);
                style
            }))
        }
        layout = layout.spacing(50);
        //
        layout =
            layout.push(button(text("重新整理").size(30)).on_press(GUIMessages::FetchPlayerInfo));
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
        String::from(PROJECT_NAME)
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNight
    }

    pub fn subscription(&self) -> iced::Subscription<GUIMessages> {
        iced::event::listen_with(|event, _status, _id| match event {
            iced::Event::Window(window_event) => match window_event {
                iced::window::Event::RedrawRequested(_) => {
                    return Some(GUIMessages::FetchPlayerInfo);
                }
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        })
    }
}
