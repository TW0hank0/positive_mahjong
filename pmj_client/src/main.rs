#[cfg(feature = "desktop")]
use iced;
#[cfg(feature = "desktop")]
use iced_core;

#[cfg(feature = "desktop")]
mod client;

#[cfg(feature = "desktop")]
use image;

#[cfg(feature = "desktop")]
use pmj_shared::shared::{self, ICON_PNG_BYTES, PROJECT_NAME};

#[cfg(feature = "desktop")]
pub const FONT_NOTO_SANS_REG: iced_core::font::Font =
    iced_core::font::Font::with_name("Noto Sans TC");

#[cfg(feature = "desktop")]
pub fn icon_init() -> Option<iced::window::Icon> {
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let icon = iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    icon
}

fn main() {
    #[cfg(feature = "desktop")]
    {
        let icon = icon_init();
        let mut window_settings = iced::window::Settings::default();
        window_settings.maximized = true;
        window_settings.icon = icon;
        window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
        window_settings.position = iced::window::Position::Centered;
        //
        let mut app_settings = iced::Settings::default();
        app_settings.id = Some(String::from(PROJECT_NAME));
        app_settings.default_text_size = iced::Pixels::from(22);
        app_settings.default_font = FONT_NOTO_SANS_REG;
        //
        let _ = iced::application(
            client::Client::new,
            client::Client::update,
            client::Client::view,
        )
        .window(window_settings)
        .settings(app_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .title(client::Client::title)
        .run();
    }
    #[cfg(not(feature = "desktop"))]
    {
        panic!("Must enable `desktop` feature on desktop platform (main.rs)!");
    }
}
