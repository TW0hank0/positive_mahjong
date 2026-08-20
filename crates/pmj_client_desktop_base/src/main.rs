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

use iced;
use image;
use tracing::{debug, error};

mod circular;
mod client;
mod easing;

use pmj_shared::shared::{self, FONT_NOTO_SANS_REG_BYTES, ICON_PNG_BYTES, PROJECT_NAME};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn icon_init() -> Option<iced::window::Icon> {
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok()
}

fn main() {
    let _guard = shared::init_tracing_fmt(String::from("pmj_client_desktop"));
    let window_settings = iced::window::Settings {
        maximized: true,
        min_size: Some(iced::Size::new(720.0, 480.0)),
        icon: icon_init(),
        position: iced::window::Position::Centered,
        ..Default::default()
    };
    let app_settings = iced::Settings {
        id: Some(format!("{} - pmj_client_desktop_base", PROJECT_NAME)),
        default_text_size: iced::Pixels::from(24),
        default_font: FONT_NOTO_SANS_REG,
        vsync: true,
        fonts: vec![std::borrow::Cow::from(FONT_NOTO_SANS_REG_BYTES)],
        ..Default::default()
    };
    let iced_result = iced::application(
        client::Client::new,
        client::Client::update,
        client::Client::view,
    )
    .window(window_settings)
    .settings(app_settings)
    .default_font(FONT_NOTO_SANS_REG)
    .title(client::Client::title)
    .theme(client::Client::theme)
    .run();
    match iced_result {
        Ok(_) => {
            debug!("iced::Result::Ok");
        }
        Err(e) => {
            error!("iced::Result::Err => {}", e);
        }
    }
}
