use serde;
use std;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestType {
    /// 需為 `positive_mahjong`
    pub app: String,
    /// 客戶端程式名
    pub client: String,
    pub data: ClientRequestDataType,
}

impl std::fmt::Display for ClientRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequstType
    app: {}
    client: {}
    data: {}",
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
            }
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataType {
    pub req_type: ActionType,
    pub data_remove_player: Option<ClientRequestDataRemovePlayerType>,
}

impl std::fmt::Display for ClientRequestDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientRequestDataType
    req_type: {}
    data_remove_player: {:?}",
            self.req_type, self.data_remove_player
        )
    }
}

impl std::default::Default for ClientRequestDataType {
    fn default() -> Self {
        Self {
            req_type: ActionType::TestConnection,
            data_remove_player: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestDataRemovePlayerType {
    pub number: u8,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseType {
    pub app: String,
    pub data: ServerResponseDataType,
    pub msg: String,
    pub is_error: bool,
}
