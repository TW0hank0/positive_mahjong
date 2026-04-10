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
    pub player_used_cards: Vec<Vec<PMJCard>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServerMessageType {
    pub msg_type: ServerMessageTypeKinds,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ServerMessageTypeKinds {
    GameStart,
    GameFinish,
    /// ChangedTurn(玩家id)
    ChangedTurn(u8),
    HandCardChange(Vec<PMJCard>),
    Error,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClientMessageType {
    pub msg_type: ClientMessageTypeKinds,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ClientMessageTypeKinds {
    ///抽牌
    GetCard,
    /// 丟牌
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
