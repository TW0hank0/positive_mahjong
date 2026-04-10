use iced;
use image;
use pmj_shared::shared;
use std::fs;

use pmj_shared::shared::{FONT_NOTO_SANS_REG_BYTES, ICON_PNG_BYTES};

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

pub fn read_server_config() -> shared::PMJServerConfig {
    if fs::exists(shared::SERVER_CONFIG_FILE_NAME).unwrap_or(false) {
        let config_str = fs::read_to_string(shared::SERVER_CONFIG_FILE_NAME).unwrap();
        serde_json::from_str(&config_str).unwrap()
    } else {
        let default_config = shared::PMJServerConfig::default();
        fs::write(
            shared::SERVER_CONFIG_FILE_NAME,
            serde_json::to_string_pretty(&default_config).unwrap(),
        )
        .ok();
        default_config
    }
}
