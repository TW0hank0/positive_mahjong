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

use tracing::{debug, error};

mod client;

use pmj_shared::shared::init_tracing_fmt;
use pmj_desktop::shared::{iced_app_settings, iced_window_settings, FONT_NOTO_SANS_REG};

fn main() {
    let _guard = init_tracing_fmt(String::from("pmj-client-desktop"));
    let iced_result = iced::application(
        client::Client::new,
        client::Client::update,
        client::Client::view,
    )
    .window(iced_window_settings())
    .settings(iced_app_settings(String::from(client::MEMBER_NAME)))
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
