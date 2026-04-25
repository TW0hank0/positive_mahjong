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

//! 通用資料

use std::fmt::Display;

use serde;

use crate::gamemodes_shared;

pub const PROJECT_NAME: &str = "positive_mahjong";
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ICON_PNG_BYTES: &[u8] = include_bytes!("../../assets/icon.png");
pub const FONT_NOTO_SANS_REG_BYTES: &[u8] =
    include_bytes!("../../assets/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");

//pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub const SERVER_PORT: u16 = 6060;

/*pub fn gui_init() -> Option<iced::window::Icon> {
    let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
    //
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let icon = iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    icon
}*/

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClientFirstConnectType {
    /// 需為 `positive_mahjong` 。
    ///
    /// 否則會拒絕連線
    pub app_name: String,
    /// 無限制。
    ///
    /// 不影響連線。
    pub client: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServerFirstConnectType {
    pub is_start: Option<bool>,
    pub is_error: bool,
    #[serde(default)]
    pub player_id: Option<u8>,
    #[serde(default)]
    pub error_type: Option<ServerFirstConnectErrorTypes>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ServerFirstConnectErrorTypes {
    TooManyPlayer,
    IpBlocked,
    Unknown,
}

impl Display for ServerFirstConnectErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooManyPlayer => "玩家數量超出限制",
            Self::IpBlocked => "Ip被封鎖",
            Self::Unknown => "伺服器端未知錯誤",
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClientConnectRequestType {
    /// 需為 `positive_mahjong`
    /// 否則會拒絕
    pub app_name: String,
    pub client: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServerConnectResponceType {
    pub gamemode: GameModes,
    pub player_id: Option<u8>,
    pub too_many_player: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct OldClientRequestType {
    /// 需為 `positive_mahjong`
    /// 否則會拒絕
    pub app: String,
    /// 客戶端程式名，無限制
    pub client: String,
    pub data: ClientRequestDataType,
    //pub game_data_v1: Option<gamemodes_shared::sharedv1_simple::GameDataV1>,
    //pub game_action_v1: Option<gamemodes_shared::sharedv1_simple::GameActions>,
    pub is_test_connection: bool,
}

impl std::default::Default for OldClientRequestType {
    fn default() -> Self {
        Self {
            app: String::from("positive_mahjong"),
            client: String::from("pmj_client"),
            data: ClientRequestDataType::default(),
            //game_data_v1: None,
            is_test_connection: true,
        }
    }
}

impl std::fmt::Display for OldClientRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequstType
    app: {}
    client: {}
    data: {:?}",
            self.app, self.client, self.data
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ActionType {
    AddPlayer,
    RemovePlayer,
    TestConnection,
    IsStart,
    SendGameAction,
    SyncGameStatus,
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AddPlayer => "新增玩家 (AddPlayer)",
                Self::RemovePlayer => "刪除玩家 (RemovePlayer)",
                Self::TestConnection => "測試連線 (TestConnection)",
                Self::IsStart => "是否開始 (IsStart)",
                Self::SendGameAction => "遊戲動作 (SendGameAction)",
                Self::SyncGameStatus => "同步遊戲資料 (SyncGameStatus)",
            }
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataType {
    pub req_action_type: ActionType,
    pub data_remove_player: Option<ClientRequestDataRemovePlayerType>,
    pub data_test_connection: Option<ClientRequestDataTestConnectionType>,
    pub data_is_start: Option<ClientRequestDataIsStartType>,
}

impl std::fmt::Display for ClientRequestDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequestDataType
    req_type: {}
    data_remove_player: {:?}
    data_test_connection: {:?}",
            self.req_action_type, self.data_remove_player, self.data_test_connection
        )
    }
}

impl std::default::Default for ClientRequestDataType {
    fn default() -> Self {
        Self {
            req_action_type: ActionType::TestConnection,
            data_remove_player: None,
            data_test_connection: None,
            data_is_start: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataRemovePlayerType {
    pub number: u8,
}

impl std::fmt::Display for ClientRequestDataRemovePlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequestDataRemovePlayerType {{ number: {} }}",
            self.number
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataTestConnectionType {
    pub number: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataIsStartType {
    pub number: u8,
}

impl std::fmt::Display for ClientRequestDataTestConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequestDataTestConnectionType {{ number: {} }}",
            self.number
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseDataAddPlayerType {
    pub number: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseDataIsStartType {
    pub is_start: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseDataTestConnectionType {
    pub msg: String,
}

/// 伺服器回應資料
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseType {
    /// 客戶端名稱
    pub app: String,
    /// 資料
    pub data: ServerResponseDataType,
    /// 訊息 (通常錯誤時才有)
    pub msg: String,
    /// 是否錯誤
    pub is_error: bool,
    /// 遊戲模式
    ///
    /// 遊戲未開始時：`None`
    ///
    /// 遊戲已開始時：`Some(GameModes)`
    pub gamemode: Option<GameModes>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum GameModes {
    Base,
    V1Simple,
    V2Better,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseDataType {
    pub data_add_player: Option<ServerResponseDataAddPlayerType>,
    pub data_test_connection: Option<ServerResponseDataTestConnectionType>,
    pub data_is_start: Option<ServerResponseDataIsStartType>,
    pub data_type: ActionType,
    /* pub gamedata_v1 */ //FIXME
}

impl std::default::Default for ServerResponseDataType {
    fn default() -> Self {
        Self {
            data_add_player: None,
            data_test_connection: None,
            data_is_start: None,
            data_type: ActionType::TestConnection,
        }
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
