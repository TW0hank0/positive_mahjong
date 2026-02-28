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

//! 資料v1 - `simple`

use serde;

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
            self.req_type, self.data_remove_player, self.data_test_connection
        )
    }
}

impl std::default::Default for ClientRequestDataType {
    fn default() -> Self {
        Self {
            req_type: ActionType::TestConnection,
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

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize,
)]
pub struct PMJCard {
    pub card_type: PMJCardTypes,
    /// - 萬、條、筒：
    ///
    /// 1~4 (1, 2, 3, 4)
    ///
    /// - 花、字：
    ///
    /// 0
    pub card_number: u8,
    /// 代表第 `card_id` 張牌
    pub card_id: u8,
}

impl std::fmt::Display for PMJCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.card_number, self.card_type)
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize,
)]
pub enum PMJCardTypes {
    TenThousand,            //萬
    Line,                   //條
    Dots,                   //筒
    Flower(PMJCardFlowers), //花
    Words(PMJCardWords),    //字
}

impl std::fmt::Display for PMJCardTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TenThousand => String::from("萬"),
                Self::Line => String::from("條"),
                Self::Dots => String::from("筒"),
                Self::Flower(flower) => format!("{}", flower),
                Self::Words(word) => format!("{}", word),
            }
        )
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize,
)]
pub enum PMJCardFlowers {
    Spring,        //春
    Summer,        //夏
    Fall,          //秋
    Winter,        //冬
    Plum,          //梅
    Orchid,        //蘭
    Bamboo,        //竹
    Chrysanthemum, //菊
}

impl PMJCardFlowers {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Spring,
            Self::Summer,
            Self::Fall,
            Self::Winter,
            Self::Plum,
            Self::Orchid,
            Self::Bamboo,
            Self::Chrysanthemum,
        ]
    }
}

impl std::fmt::Display for PMJCardFlowers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Spring => "春",
                Self::Summer => "夏",
                Self::Fall => "秋",
                Self::Winter => "冬",
                Self::Plum => "梅",
                Self::Orchid => "蘭",
                Self::Bamboo => "竹",
                Self::Chrysanthemum => "菊",
            }
        )
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize,
)]
pub enum PMJCardWords {
    East,        //東
    South,       //南
    West,        //西
    North,       //北
    RedDragon,   //中 (紅中)
    GreenDragon, //青發
    WhiteDragon, //白板
}

impl PMJCardWords {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::East,
            Self::South,
            Self::West,
            Self::North,
            Self::RedDragon,
            Self::GreenDragon,
            Self::WhiteDragon,
        ]
    }
}

impl std::fmt::Display for PMJCardWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::East => "東",
                Self::South => "南",
                Self::West => "西",
                Self::North => "北",
                Self::RedDragon => "中",
                Self::GreenDragon => "青發",
                Self::WhiteDragon => "白板",
            }
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GameActions {
    WaitRound(GameActionWaitRound),
    PlayerRound(GameActionPlayerRound),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum GameActionWaitRound {
    ///補花
    ReplacingAFlower(PMJCard),
    ///吃
    Eat(PMJCard),
    ///碰
    Triplet(PMJCard),
    ///明槓
    ExposedKong(PMJCard),
    ///暗槓
    ConcealedKong(PMJCard),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum GameActionAfter {
    ///丟牌
    Throw(PMJCard),
}

impl std::fmt::Display for GameActionAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Throw(_) => "丟牌",
            }
        )
    }
}

impl std::fmt::Display for GameActionWaitRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ReplacingAFlower(_) => "補花",
                Self::Eat(_) => "吃",
                Self::Triplet(_) => "碰",
                Self::ExposedKong(_) => "明槓",
                Self::ConcealedKong(_) => "暗槓",
            }
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum GameActionPlayerRound {
    ///摸牌
    DrawATile,
}

impl std::fmt::Display for GameActionPlayerRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DrawATile => "摸牌",
            }
        )
    }
}

#[derive(Debug, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize)]
pub struct PMJPlayer {
    pub ip: std::net::SocketAddr,
    pub number: u8,
    pub cards: Vec<PMJCard>,
}

impl std::cmp::PartialEq for PMJPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip && self.number == other.number
    }
}

impl std::cmp::Eq for PMJPlayer {}

impl std::fmt::Display for PMJPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PMJPlayer:
    ip: {}
    number: {}
    cards: {:?}
",
            self.ip.to_string(),
            self.number.to_string(),
            self.cards
        )
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
