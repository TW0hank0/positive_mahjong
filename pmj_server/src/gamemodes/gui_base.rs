//! Base玩法的GUI

use std::sync::{self, Arc, RwLock};

use iced::{
    self,
    widget::{self, Column, Row, container, text},
};
use image;
use local_ip_address;

use crate::gamemodes::mode_base;

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
    backend: Arc<RwLock<mode_base::PositiveMahjong>>,
    local_ipv4_address: std::net::IpAddr,
    local_ipv6_address: std::net::IpAddr,
    msg: String,
}

impl ServerGUI {
    fn new() -> Self {
        let ipv4_address = local_ip_address::local_ip().unwrap();
        let ipv6_address = local_ip_address::local_ipv6().unwrap();
        let backend = mode_base::main_base(true);
        Self {
            backend: backend,
            local_ipv4_address: ipv4_address,
            local_ipv6_address: ipv6_address,
            msg: String::new(),
        }
    }

    fn update(&mut self, msg: GUIMessages) {
        match msg {
            GUIMessages::StartGame => match self.backend.write() {
                Ok(mut guard) => {
                    guard.start_game();
                }
                Err(err) => self.msg = format!("{}\n---\n{}\n", self.msg, err),
            },
        }
    }

    fn view(&self) -> iced::widget::Column<'_, GUIMessages> {
        let mut layout: iced::widget::Column<'_, GUIMessages> = Column::new();
        //
        let mut ip_bar_layout = Row::new();
        ip_bar_layout =
            ip_bar_layout.push(text(format!("Ipv4: {}", self.local_ipv4_address)).size(28));
        ip_bar_layout = ip_bar_layout.spacing(40);
        ip_bar_layout = ip_bar_layout
            .push(text(format!("Ipv6: {}", self.local_ipv6_address)).size(iced::Pixels::from(28)));
        let ip_bar_container = container(ip_bar_layout).style(|_theme| {
            iced::widget::container::Style::default()
                .background(iced::Background::Color(iced::Color::from_rgb8(99, 99, 99)))
                .border(iced::border::Border::default().rounded(iced::border::radius(10.0)))
        });
        layout = layout.push(ip_bar_container);
        //
        layout =
            layout.push(widget::button(widget::text("Start")).on_press(GUIMessages::StartGame));
        return layout;
    }
}
