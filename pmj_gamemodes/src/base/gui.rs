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
    self,
    widget::{self, Column, Row, container, text},
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
    iced::application(ServerGUI::new, ServerGUI::update, ServerGUI::view).run()
}

#[derive(Debug, Clone, Copy)]
enum GUIMessages {
    StartGame,
}

#[derive(Debug)]
struct ServerGUI {
    backend: Arc<RwLock<base::mode::PositiveMahjong>>,
    local_ipv4_address: std::net::IpAddr,
    local_ipv6_address: std::net::IpAddr,
    msg: String,
    is_start: bool,
}

impl ServerGUI {
    fn new() -> Self {
        let ipv4_address = local_ip_address::local_ip().unwrap();
        let ipv6_address = local_ip_address::local_ipv6().unwrap();
        let backend = base::mode::main_base(true);
        Self {
            backend: backend,
            local_ipv4_address: ipv4_address,
            local_ipv6_address: ipv6_address,
            msg: String::new(),
            is_start: false,
        }
    }

    fn update(&mut self, msg: GUIMessages) {
        match msg {
            GUIMessages::StartGame => {
                self.is_start = true;
                let thread_backend = sync::Arc::clone(&self.backend);
                let _ = thread::spawn(move || match thread_backend.write() {
                    Ok(mut guard) => {
                        guard.start_game();
                    }
                    Err(_err) => { /* TODO: error handle */ }
                });
            }
        }
    }

    fn view(&self) -> iced::widget::Column<'_, GUIMessages> {
        let mut layout: iced::widget::Column<'_, GUIMessages> = Column::new().spacing(20);
        //
        let mut ip_bar_layout = Row::new().spacing(30);
        ip_bar_layout =
            ip_bar_layout.push(text(format!("Ipv4: {}", self.local_ipv4_address)).size(28));
        ip_bar_layout = ip_bar_layout.spacing(40);
        ip_bar_layout = ip_bar_layout
            .push(text(format!("Ipv6: {}", self.local_ipv6_address)).size(iced::Pixels::from(28)))
            .spacing(20);
        let ip_bar_container = container(ip_bar_layout).style(|_theme| {
            iced::widget::container::Style::default()
                .background(iced::Background::Color(iced::Color::from_rgb8(99, 99, 99)))
                .border(iced::border::Border::default().rounded(iced::border::radius(10.0)))
        });
        layout = layout.push(ip_bar_container).spacing(80);
        //
        if !self.is_start {
            let start_button = widget::button(widget::text("Start").size(34))
                .on_press(GUIMessages::StartGame)
                /* .style(|_theme, _status| {
                    let mut button_style = widget::button::Style::default();
                    button_style.border = iced::border::rounded(iced::border::radius(12.0));
                    button_style.background =
                        Some(iced::Background::Color(iced::Color::from_rgb8(99, 99, 99)));
                    button_style
                })*/;
            layout = layout.push(start_button);
        }
        return layout;
    }
}
