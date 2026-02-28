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

use serde;

use crate::gamemodes_shared;

pub const SERVER_PORT: u16 = 6060;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestType {
    /// 需為 `positive_mahjong`
    /// 否則會拒絕
    pub app: String,
    /// 客戶端程式名，無限制
    pub client: String,
    pub data: ClientRequestDataType,
    pub game_data_v1: Option<gamemodes_shared::sharedv1_simple::GameDataV1>,
    //pub game_action_v1: Option<gamemodes_shared::sharedv1_simple::GameActions>,
    pub is_test_connection: bool,
}

impl std::default::Default for ClientRequestType {
    fn default() -> Self {
        Self {
            app: String::from("positive_mahjong"),
            client: String::from("pmj_client"),
            data: ClientRequestDataType::default(),
            game_data_v1: None,
            is_test_connection: true,
        }
    }
}

impl std::fmt::Display for ClientRequestType {
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
    V1Simple,
    V2Better,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseDataType {
    pub data_add_player: Option<ServerResponseDataAddPlayerType>,
    pub data_test_connection: Option<ServerResponseDataTestConnectionType>,
    pub data_is_start: Option<ServerResponseDataIsStartType>,
    pub data_type: ActionType,
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
