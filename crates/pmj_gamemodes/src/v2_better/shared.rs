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

//! V2Better 資料

use std::{self, fmt::Display, net, sync};
use tungstenite;

pub const MAX_PLAYER_COUNT: u8 = 4;
/// Websocket 連線類型
type WsConnection = tungstenite::WebSocket<net::TcpStream>;

#[derive(Debug, Clone)]
pub struct PMJPlayer {
    pub player_ip_addr: net::IpAddr,
    pub player_id: u8,
    pub player_ws: sync::Arc<sync::RwLock<WsConnection>>,
    /// 可使用的牌
    pub player_hand_cards: Vec<PMJCard>,
    /// 存放使用過的牌
    pub player_used_cards: Vec<(Vec<PMJCard>, GameActions)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ServerMessage {
    pub msg_type: ServerMsgKinds,
    pub room_msg: Option<ServerRoomMsg>,
    pub game_msg: Option<ServerGameMsg>,
}

impl Default for ServerMessage {
    fn default() -> Self {
        Self {
            msg_type: ServerMsgKinds::RoomMsg,
            room_msg: None,
            game_msg: None,
        }
    }
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ServerMsgKinds {
    GameMsg,
    RoomMsg,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ServerRoomMsg {
    msg_type: ServerGameMsgKinds,
    /// Option<(玩家識別碼, 發言內容)>
    info_player_say: Option<(u8, String)>,
    info_root_say: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ServerRoomMsgKinds {
    // 玩家發言
    PlayerSay,
    // 伺服主發言
    RootSay
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ServerGameMsg {
    pub msg_type: ServerGameMsgKinds,
    pub info_hand_card_change: Option<Vec<PMJCard>>,
    pub info_error: Option<String>,
    /// 來自你和其他玩家的動作 Option<(玩家Id, 動作)>
    pub info_player_action: Option<(u8, GameActions)>,
    pub info_get_card: Option<PMJCard>,
    pub info_change_turn: Option<u8>,
}

impl Default for ServerGameMsg {
    fn default() -> Self {
        Self {
            msg_type: ServerGameMsgKinds::Error,
            info_player_action: None,
            info_hand_card_change: None,
            info_error: Some(String::from("Default `info_error` value.")),
            info_change_turn: None,
            info_get_card: None,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ServerGameMsgKinds {
    GameStart,
    GameFinish,
    ChangedTurn,
    /// 手牌變動
    HandCardChange,
    Error,
    /// 玩家動作
    PlayerAction,
    GetCard,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ClientMessage {
    msg_type: ClientMsgKinds,
    game_msg: Option<ClientGameMsg>
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ClientMsgKinds {
    GameMsg,
    RoomMsg,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ClientGameMsg {
    pub msg_type: ClientGameMsgKinds,
    pub info_game_action: Option<GameActions>,
    ///丟牌
    pub info_throw_card: Option<PMJCard>,
    ///補花
    pub info_replace_a_flower: Option<PMJCard>,
    ///吃
    pub info_eat: Option<(PMJCard, PMJCard)>,
    ///碰
    pub info_triplet: Option<(PMJCard, PMJCard)>,
    ///明槓
    pub info_exposed_kong: Option<(PMJCard, PMJCard, PMJCard)>,
    ///暗槓
    pub info_concealed_kong: Option<(PMJCard, PMJCard, PMJCard)>,
}

impl Default for ClientGameMsg {
    fn default() -> Self {
        Self {
            msg_type: ClientGameMsgKinds::GameAction,
            info_game_action: None,
            info_throw_card: None,
            info_replace_a_flower: None,
            info_concealed_kong: None,
            info_eat: None,
            info_exposed_kong: None,
            info_triplet: None,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ClientGameMsgKinds {
    GameAction,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum GameActions {
    ///抽牌
    GetCard,
    ///丟牌
    ThrowCard,
    ///吃
    Eat,
    ///碰
    Triplet,
    ///明槓
    ExposedKong,
    ///暗槓
    ConcealedKong,
    ///補花
    ReplaceFlower,
}

/// 卡牌
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct PMJCard {
    /// 種類
    pub card_type: PMJCardType,
    /// 此卡牌第`card_id`張
    pub card_id: u8,
    ///萬
    pub info_ten_thousand: Option<u8>,
    ///條
    pub info_line: Option<u8>,
    ///筒
    pub info_dots: Option<u8>,
    ///花
    pub info_flower: Option<PMJCardFlowerType>,
    /// 字
    pub info_words: Option<PMJCardWordsType>,
}

impl Display for PMJCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", match self.card_type {
            PMJCardType::Dots => {self.info_dots.unwrap().to_string()}
            PMJCardType::Flower => {self.info_flower.clone().unwrap().to_string()}
            PMJCardType::Line => {self.info_line.unwrap().to_string()}
            PMJCardType::TenThousand => {self.info_ten_thousand.unwrap().to_string()}
            PMJCardType::Words => {self.info_words.clone().unwrap().to_string()}
        }, match self.card_type {
            PMJCardType::Dots => {"筒"}
            PMJCardType::Flower => {""}
            PMJCardType::Line => {"條"}
            PMJCardType::TenThousand => {"萬"}
            PMJCardType::Words => {""}
        }))
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum PMJCardType {
    ///萬
    TenThousand,
    ///條
    Line,
    ///筒
    Dots,
    ///花
    Flower,
    ///字
    Words,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum PMJCardFlowerType {
    ///春
    Spring,
    ///夏
    Summer,
    ///秋
    Fall,
    ///冬
    Winter,
    ///梅
    Plum,
    ///蘭
    Orchid,
    ///竹
    Bamboo,
    ///菊
    Chrysanthemum,
}

impl Display for PMJCardFlowerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Spring => "春",
            Self::Summer => "夏",
            Self::Fall => "秋",
            Self::Winter => "冬",
            Self::Plum => "梅",
            Self::Orchid => "蘭",
            Self::Bamboo => "竹",
            Self::Chrysanthemum => "菊",
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum PMJCardWordsType {
    ///東
    East,
    ///南
    South,
    ///西
    West,
    ///北
    North,
    ///紅中
    RedDragon,
    ///青發
    GreenDragon,
    ///白板
    WhiteDragon,
}

impl std::fmt::Display for PMJCardWordsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::East => "東",
            Self::South => "南",
            Self::West => "西",
            Self::North => "北",
            Self::RedDragon => "中",
            Self::GreenDragon => "青發",
            Self::WhiteDragon => "白板",
        })
    }
}
