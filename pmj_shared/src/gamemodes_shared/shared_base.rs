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

use std::fmt::Display;
use std::net::TcpStream;
use std::sync;
use tungstenite::WebSocket;

//use serde;

pub const MAX_PLAYER_COUNT: u8 = 4;

#[derive(Debug)]
pub struct PMJPlayer {
    pub player_ip_addr: std::net::IpAddr,
    pub player_id: u8,
    pub player_ws: sync::Arc<sync::RwLock<WebSocket<TcpStream>>>,
    /// 可使用的牌
    pub player_hand_cards: Vec<PMJCard>,
    /// 存放使用過的牌，例：碰、槓、吃
    ///
    /// 格式為：
    ///
    /// [ [A, A, A], [B, C, D] ]
    ///   ^碰~~~~     ^吃~~~~~
    pub player_used_cards: Vec<Vec<PMJCard>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServerMessageType {
    pub msg_type: ServerMessageTypeKinds,
    pub info_change_turn: Option<u8>,
    pub info_hand_card_change: Option<Vec<PMJCard>>,
    pub info_error: Option<String>,
}

impl Default for ServerMessageType {
    fn default() -> Self {
        Self {
            msg_type: ServerMessageTypeKinds::Error,
            info_change_turn: None,
            info_hand_card_change: None,
            info_error: None,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ServerMessageTypeKinds {
    GameStart,
    GameFinish,
    ChangedTurn,
    /// 手牌變動
    HandCardChange,
    Error,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClientMessageType {
    pub msg_type: ClientMessageTypeKinds,
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

impl Default for ClientMessageType {
    fn default() -> Self {
        Self {
            msg_type: ClientMessageTypeKinds::GetCard,
            info_throw_card: None,
            info_replace_a_flower: None,
            info_concealed_kong: None,
            info_eat: None,
            info_exposed_kong: None,
            info_triplet: None,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ClientMessageTypeKinds {
    ///抽牌
    GetCard,
    ///丟牌
    ThrowCard,
    ///補花
    ReplacingAFlower,
    ///吃
    Eat,
    ///碰
    Triplet,
    ///明槓
    ExposedKong,
    ///暗槓
    ConcealedKong,
}

/*pub fn need_throw_after_action(act: ClientMessageTypeKinds) -> bool {
    match act {
        ClientMessageTypeKinds::GetCard => true,
        ClientMessageTypeKinds::ConcealedKong => true,
        ClientMessageTypeKinds::ExposedKong => true,
        ClientMessageTypeKinds::Eat => true,
        ClientMessageTypeKinds::Triplet => true,
        _ => false,
    }
}*/

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum GameTurnTypes {
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

/// Base玩法的卡牌
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
