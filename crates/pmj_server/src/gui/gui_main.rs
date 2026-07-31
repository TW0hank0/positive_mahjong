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

use std::{env, fs};

use iced;
use positive_tool_rs;
use tracing::{debug, error, info, warn};

use pmj_shared;

mod base;

fn main() {
    if !fs::exists(
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("pmj_logs"),
    )
    .unwrap_or(false)
    {
        fs::create_dir(
            env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("pmj_logs"),
        )
        .ok();
    }
    let _guard = positive_tool_rs::pt::init_tracing(
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("pmj_logs"),
        Some(String::from("pmj_server")),
    );
    let config = pmj_shared::shared::read_server_config();
    let iced_result: iced::Result;
    match config.gamemode {
        pmj_shared::shared::GameModes::Base => {
            info!("config.gamemode = GameModes::Base");
            iced_result = base::main();
        }
        pmj_shared::shared::GameModes::V1Simple => {
            warn!("還未支援！");
            std::process::exit(1);
        }
        pmj_shared::shared::GameModes::V2Better => {
            warn!("還未支援！");
            std::process::exit(1);
        }
    }
    match iced_result {
        Ok(_) => {
            debug!("iced::Result::Ok");
        }
        Err(e) => {
            error!("iced::Result::Err => {}", e);
        }
    }
}
