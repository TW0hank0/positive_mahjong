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
