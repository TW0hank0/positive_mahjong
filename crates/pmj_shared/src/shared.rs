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

//! 通用資料

use std::{env, fmt, fs};

use positive_tool_rs;
use serde;
use serde_json;
use tracing::debug;
use tracing_appender;

pub const PROJECT_NAME: &str = "positive_mahjong";
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ICON_PNG_BYTES: &[u8] = include_bytes!("../../../assets/icon/icon.png");
pub const ICON_SVG_BYTES: &[u8] = include_bytes!("../../../assets/icon/icon.svg");

pub const FONT_NOTO_SANS_REG_BYTES: &[u8] =
    include_bytes!("../../../assets/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");
pub const FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES: &[u8] = include_bytes!(
    "../../../assets/material_symbols/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf"
);

pub const SERVER_PORT: u16 = 6060;
pub const DEFAULT_GAMEMODE: GameModes = GameModes::Base;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClientConnectRequestType {
    /// 需為 `positive_mahjong`
    /// 否則會拒絕
    /// TODO: 一個更好的做法
    pub app_name: String,
    /// 無限制
    pub client: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServerConnectResponceType {
    pub gamemode: GameModes,
    pub player_id: Option<u8>,
    pub too_many_player: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum GameModes {
    Base,
    V1Simple,
    V2Better,
}

impl fmt::Display for GameModes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Base => "Base",
            Self::V1Simple => "V1Simple",
            Self::V2Better => "V2Better",
        })
    }
}

pub const SERVER_CONFIG_FILE_NAME: &str = "pmj_server_config.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PMJServerConfig {
    pub gamemode: GameModes,
}

impl Default for PMJServerConfig {
    fn default() -> Self {
        Self {
            gamemode: GameModes::Base,
        }
    }
}

pub fn read_server_config() -> PMJServerConfig {
    if fs::exists(SERVER_CONFIG_FILE_NAME).unwrap_or(false) {
        let config_str = fs::read_to_string(SERVER_CONFIG_FILE_NAME).unwrap();
        serde_json::from_str(&config_str).unwrap()
    } else {
        let default_config = PMJServerConfig::default();
        fs::write(
            SERVER_CONFIG_FILE_NAME,
            serde_json::to_string_pretty(&default_config).unwrap(),
        )
        .ok();
        default_config
    }
}

pub fn init_tracing_fmt(member_name: String) -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir_path = dirs::data_local_dir()
        .unwrap_or(env::current_dir().unwrap())
        .join("positive_mahjong")
        .join(member_name.clone());
    if !fs::exists(&log_dir_path).unwrap_or(false) {
        fs::create_dir_all(&log_dir_path).ok();
    }
    let guard = positive_tool_rs::pt::init_tracing(log_dir_path, Some(member_name));
    debug!("成功初始化日誌。");
    guard
}
