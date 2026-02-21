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

use local_ip_address;
use serde_json;
use std;
use std::sync;
use tiny_http;

use rand;
use rand::{prelude::SliceRandom, seq::IndexedRandom};

use positive_mahjong::shared::{self, GameActionAfter, GameActionPlayerRound, GameActionWaitRound};
use positive_mahjong::shared::{PMJCard, PMJCardFlowers, PMJCardTypes, PMJCardWords};

fn main() {
    println!("ipv4: {}", local_ip_address::local_ip().unwrap());
    println!("ipv6: {}", local_ip_address::local_ipv6().unwrap());
    //
    let backend = sync::Arc::new(sync::RwLock::new(PositiveMahjong::new()));
    let server_addr_ipv4 = std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::UNSPECIFIED,
        shared::SERVER_PORT,
    ));
    let server_addr_ipv6 = std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::UNSPECIFIED,
        shared::SERVER_PORT,
        0,
        0,
    ));
    let mut servers = Vec::new();
    servers.push(handle_server(server_addr_ipv4, sync::Arc::clone(&backend)));
    servers.push(handle_server(server_addr_ipv6, sync::Arc::clone(&backend)));
    println!("start? (press <enter>)");
    let mut bind = String::new();
    std::io::stdin().read_line(&mut bind).ok();
    let backend_arc = sync::Arc::clone(&backend);
    {
        let mut guard = backend_arc.write().unwrap();
        guard.start_game();
    }
    {
        let guard = backend_arc.read().unwrap();
        println!("{}", guard);
    }
    let duration_2sec = std::time::Duration::from_secs(2);
    for server in servers {
        loop {
            if server.is_finished() {
                break;
            } else {
                std::thread::sleep(duration_2sec);
            }
        }
    }
}

fn handle_server(
    addr: std::net::SocketAddr,
    backend: sync::Arc<sync::RwLock<PositiveMahjong>>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        /* let server = tiny_http::Server::new(tiny_http::ServerConfig {
            addr: tiny_http::ConfigListenAddr::IP(vec![addr]),
            ssl: None,
        })
        .unwrap(); */
        let server = tiny_http::Server::http(addr).unwrap();
        for request in server.incoming_requests() {
            let arc_backend = sync::Arc::clone(&backend);
            std::thread::spawn(move || {
                handle_request(request, arc_backend);
            });
        }
    })
}

fn handle_request(
    mut request: tiny_http::Request,
    backend: sync::Arc<sync::RwLock<PositiveMahjong>>,
) {
    let mut content_string = String::new();
    request
        .as_reader()
        .read_to_string(&mut content_string)
        .unwrap();
    let content_data_result: Result<shared::ClientRequestType, serde_json::Error> =
        serde_json::from_str(&content_string);
    match content_data_result {
        Ok(content_data) => {
            println!("{}", content_data);
            if content_data.app == "positive_mahjong" {
                match content_data.data.req_type {
                    shared::ActionType::AddPlayer => {
                        let mut guard = backend.write().unwrap();
                        match guard.add_player(request.remote_addr().unwrap().clone()) {
                            Either::Left(e) => {
                                let response_data = shared::ServerResponseDataType {
                                    data_add_player: None,
                                    data_type: content_data.data.req_type,
                                    ..Default::default()
                                };
                                let response = tiny_http::Response::from_string(
                                    serde_json::to_string(&shared::ServerResponseType {
                                        app: content_data.app,
                                        data: response_data,
                                        msg: e,
                                        is_error: true,
                                    })
                                    .unwrap(),
                                );
                                request.respond(response).ok();
                            }
                            Either::Right(number) => {
                                let response_data = shared::ServerResponseDataType {
                                    data_add_player: Some(
                                        shared::ServerResponseDataAddPlayerType { number: number },
                                    ),
                                    data_type: content_data.data.req_type,
                                    ..Default::default()
                                };
                                let response = tiny_http::Response::from_string(
                                    serde_json::to_string(&shared::ServerResponseType {
                                        app: content_data.app,
                                        data: response_data,
                                        msg: String::new(),
                                        is_error: false,
                                    })
                                    .unwrap(),
                                );
                                request.respond(response).ok();
                            }
                        }
                    }
                    shared::ActionType::TestConnection => {
                        let response_data = shared::ServerResponseDataType {
                            data_test_connection: Some(
                                shared::ServerResponseDataTestConnectionType {
                                    msg: String::from(format!(
                                        "Hello to client {}",
                                        content_data.client
                                    )),
                                },
                            ),
                            ..Default::default()
                        };
                        let response = tiny_http::Response::from_string(
                            serde_json::to_string(&shared::ServerResponseType {
                                app: content_data.app,
                                data: response_data,
                                msg: String::new(),
                                is_error: false,
                            })
                            .unwrap(),
                        );
                        request.respond(response).ok();
                    }
                    shared::ActionType::RemovePlayer => {
                        let mut guard = backend.write().unwrap();
                        match guard.remove_player(
                            request.remote_addr().unwrap().clone(),
                            content_data.data.data_remove_player.unwrap().number,
                        ) {
                            Either::Left(e) => {
                                let response_data = shared::ServerResponseDataType {
                                    data_add_player: None,
                                    data_type: content_data.data.req_type,
                                    ..Default::default()
                                };
                                let response = tiny_http::Response::from_string(
                                    serde_json::to_string(&shared::ServerResponseType {
                                        app: content_data.app,
                                        data: response_data,
                                        msg: e,
                                        is_error: true,
                                    })
                                    .unwrap(),
                                );
                                request.respond(response).ok();
                            }
                            Either::Right(_) => {
                                let response_data = shared::ServerResponseDataType {
                                    data_type: content_data.data.req_type,
                                    ..Default::default()
                                };
                                let response = tiny_http::Response::from_string(
                                    serde_json::to_string(&shared::ServerResponseType {
                                        app: content_data.app,
                                        data: response_data,
                                        msg: String::new(),
                                        is_error: false,
                                    })
                                    .unwrap(),
                                );
                                request.respond(response).ok();
                            }
                        }
                    }
                    shared::ActionType::IsStart => {
                        let guard = backend.read().unwrap();
                        println!("{}", guard);
                        let is_start = guard.is_start();
                        let response = tiny_http::Response::from_string(
                            serde_json::to_string(&shared::ServerResponseType {
                                app: content_data.app,
                                data: shared::ServerResponseDataType {
                                    data_type: content_data.data.req_type,
                                    data_is_start: Some(shared::ServerResponseDataIsStartType {
                                        is_start: is_start,
                                    }),
                                    ..Default::default()
                                },
                                msg: String::new(),
                                is_error: false,
                            })
                            .unwrap(),
                        );
                        request.respond(response).ok();
                    }
                }
            } else {
                let response = tiny_http::Response::from_string("這是positive_mahjong的server！");
                request.respond(response).ok();
            }
        }
        Err(e) => {
            let response = tiny_http::Response::from_string(e.to_string());
            request.respond(response).ok(); //TODO: log it
        }
    }
    let guard = backend.read().unwrap();
    println!("{}", guard);
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

struct PositiveMahjong {
    players: Vec<PMJPlayer>,
    player_count: u8,
    max_player_count: u8,
    is_start: bool,
    unuse_cards: Vec<PMJCard>,
    game_status: GameStatus,
    msg_queue_wait: sync::Arc<sync::RwLock<Vec<PlayerWaitRoundAction>>>,
    msg_queue_player: sync::Arc<sync::RwLock<Vec<PlayerPlayerRoundAction>>>,
    msg_queue_after: sync::Arc<sync::RwLock<Vec<AfterAction>>>,
}

#[derive(Debug)]
pub struct PlayerWaitRoundAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionWaitRound,
}

impl std::cmp::PartialEq for PlayerWaitRoundAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for PlayerWaitRoundAction {}

#[derive(Debug)]
pub struct AfterAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionAfter,
}

impl std::cmp::PartialEq for AfterAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for AfterAction {}

#[derive(Debug)]
pub struct PlayerPlayerRoundAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionPlayerRound,
}

impl std::cmp::PartialEq for PlayerPlayerRoundAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for PlayerPlayerRoundAction {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GameStatus {
    pub rounds_count: u16,
    pub round_type: GameRoundType,
    /// 當 `round_type` 是 `GameRoundType::WaitRound` 時，
    /// `current_player_number` 是最後一個動作的玩家的 `player_number`
    pub current_player_number: u8,
    /// 回合是否結束
    pub is_round_finish: bool,
    /// 遊戲是否結束
    pub is_game_finish: bool,
    /// 最後一個丟的牌
    pub last_throw_card: Option<PMJCard>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum GameRoundType {
    /// 玩家時間
    ///
    /// 可以：抽牌
    ///
    /// 不可：碰、吃、槓、暗槓、明槓、補花
    PlayerRoound,
    /// 等待時間
    ///
    /// 可以：碰、吃、槓、暗槓、明槓、補花
    ///
    /// 不可：抽牌
    WaitRound,
}

impl std::fmt::Display for PositiveMahjong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PositiveMahjong
    players: {:#?}
    player_count: {}
    max_player_count: {}",
            self.players,
            self.player_count.clone(),
            self.max_player_count
        )
    }
}

#[derive(Debug, PartialOrd, Ord, Clone)]
struct PMJPlayer {
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

impl PositiveMahjong {
    pub fn new() -> Self {
        let mut unuse_cards = Vec::new();
        for card_type in vec![
            PMJCardTypes::TenThousand,
            PMJCardTypes::Line,
            PMJCardTypes::Dots,
        ] {
            for number in 1..10 {
                for card_id in 1..5 {
                    unuse_cards.push(PMJCard {
                        card_type: card_type,
                        card_number: number,
                        card_id: card_id,
                    });
                }
            }
        }
        for word in PMJCardWords::get_all() {
            for card_id in 1..5 {
                unuse_cards.push(PMJCard {
                    card_type: PMJCardTypes::Words(word),
                    card_number: 0,
                    card_id: card_id,
                });
            }
        }
        for flower in PMJCardFlowers::get_all() {
            unuse_cards.push(PMJCard {
                card_type: PMJCardTypes::Flower(flower),
                card_number: 0,
                card_id: 1,
            });
        }
        //
        Self {
            players: Vec::new(),
            player_count: 0,
            max_player_count: 4,
            is_start: false,
            unuse_cards: unuse_cards,
            game_status: GameStatus {
                rounds_count: 0,
                round_type: GameRoundType::WaitRound,
                current_player_number: 0,
                is_round_finish: false,
                is_game_finish: false,
                last_throw_card: None,
            },
            msg_queue_player: sync::Arc::new(sync::RwLock::new(Vec::new())),
            msg_queue_wait: sync::Arc::new(sync::RwLock::new(Vec::new())),
            msg_queue_after: sync::Arc::new(sync::RwLock::new(Vec::new())),
        }
    }

    pub fn add_player(&mut self, ip: std::net::SocketAddr) -> Either<String, u8> {
        if self.player_count >= self.max_player_count {
            return Either::Left(String::from("人數已達上限！"));
        } else if self.is_start {
            return Either::Left(String::from("遊戲已開始！"));
        } else {
            self.player_count += 1;
            self.players.push(PMJPlayer {
                ip: ip,
                number: self.player_count,
                cards: Vec::new(),
            });
            return Either::Right(self.player_count.clone());
        }
    }

    pub fn remove_player(&mut self, ip: std::net::SocketAddr, number: u8) -> Either<String, ()> {
        if self.is_start {
            return Either::Left(String::from("遊戲已開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: ip,
            number: number,
            cards: Vec::new(),
        }) {
            self.players.retain(|x: &PMJPlayer| {
                x.clone()
                    != PMJPlayer {
                        ip: ip,
                        number: number,
                        cards: Vec::new(),
                    }
            });
            self.player_count -= 1;
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    pub fn is_start(&self) -> bool {
        return self.is_start.clone();
    }

    pub fn start_game(&mut self) {
        let mut rng = rand::rng();
        self.unuse_cards.shuffle(&mut rng);
        //
        for _ in 0..4 {
            for player_number in 0..self.players.len() {
                /* let player = self.players.get_mut(player_number).unwrap();
                for _ in 0..4 {
                    let card = self.unuse_cards.choose(&mut rng).unwrap().clone();
                    let mut index = 0;
                    for unuse_card in self.unuse_cards.iter() {
                        if unuse_card.clone() == card.clone() {
                            break;
                        } else {
                            index += 1;
                        }
                    }
                    self.unuse_cards.remove(index);
                    player.cards.push(card);
                } */
                self.get_one_unuse_card(player_number);
            }
        }
        //莊家多一張
        self.get_one_unuse_card(0);
        //
        //TODO:等待過補
        self.game_loop();
    }

    pub fn replacing_a_flower(
        &mut self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let player = self.players.get_mut(player_number as usize).unwrap();
            let mut flower_card_indexes: Vec<usize> = Vec::new();
            let card_index: usize = 0;
            for card in player.cards.iter() {
                match card.card_type {
                    PMJCardTypes::Flower(_) => {
                        flower_card_indexes.push(card_index.clone());
                    }
                    _ => {} //其他忽略
                }
            }
            if !flower_card_indexes.is_empty() {
                return Either::Left(String::from("無花可補！"));
            } else {
                for index in flower_card_indexes.iter() {
                    player.cards.remove(index.clone());
                }
                for _ in 0..flower_card_indexes.len() {
                    self.get_one_unuse_card(player_number as usize);
                }
                return Either::Right(());
            }
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    /// for client
    pub fn get_cards(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
    ) -> Either<String, Vec<PMJCard>> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let player = self.players.get(player_number as usize).unwrap();
            return Either::Right(player.cards.clone());
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    /// for client
    pub fn get_game_status(&self) -> GameStatus {
        return self.game_status.clone();
    }

    pub fn game_loop(&mut self) {
        loop {
            for round_player_number in 0..self.players.len() {
                for _round_type in vec![GameRoundType::PlayerRoound, GameRoundType::WaitRound] {
                    if !self.game_status.is_game_finish {
                        match self.game_status.round_type {
                            GameRoundType::PlayerRoound => {
                                self.handle_game_round_player(round_player_number as u8);
                            }
                            GameRoundType::WaitRound => {
                                self.handle_game_round_wait(round_player_number as u8);
                            }
                        }
                        self.change_game_round_type();
                    } else {
                        break;
                    }
                }
                if self.game_status.is_game_finish {
                    break;
                }
            }
            if self.game_status.is_game_finish {
                println!("遊戲已結束！");
                break;
            } else {
                self.game_status.rounds_count += 1;
            }
        }
    }

    fn change_game_round_type(&mut self) {
        match self.game_status.round_type {
            GameRoundType::PlayerRoound => {
                self.game_status.round_type = GameRoundType::WaitRound;
            }
            GameRoundType::WaitRound => {
                self.game_status.round_type = GameRoundType::PlayerRoound;
            }
        }
    }

    /// for client
    pub fn player_round_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionPlayerRound,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_player);
            let mut guard = queue_arc.write().unwrap();
            guard.push(PlayerPlayerRoundAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn wait_round_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionWaitRound,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_wait);
            let mut guard = queue_arc.write().unwrap();
            guard.push(PlayerWaitRoundAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn player_round_after_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionAfter,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_after);
            let mut guard = queue_arc.write().unwrap();
            guard.push(AfterAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn wait_round_after_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionAfter,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_after);
            let mut guard = queue_arc.write().unwrap();
            guard.push(AfterAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    fn handle_game_round_player(&mut self, player_number: u8) {
        let duration = std::time::Duration::from_secs(1);
        {
            let queue_arc_player = sync::Arc::clone(&self.msg_queue_player);
            let mut exit_loop: bool = false;
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_player.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionPlayerRound::DrawATile => {
                                    self.get_one_unuse_card(player_number as usize);
                                    exit_loop = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            {
                let mut guard = queue_arc_player.write().unwrap();
                guard.clear();
            }
        }
        //
        {
            let mut exit_loop: bool = false;
            let queue_arc_after = sync::Arc::clone(&self.msg_queue_after);
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_after.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionAfter::Throw(card) => {
                                    if self
                                        .players
                                        .get(player_number as usize)
                                        .unwrap()
                                        .cards
                                        .contains(&card)
                                    {
                                        let player =
                                            self.players.get_mut(player_number as usize).unwrap();
                                        let mut index: usize = 0;
                                        //FIXME:不太可能出現的邏輯問題
                                        for i in player.cards.iter() {
                                            if i == &card {
                                                break;
                                            } else {
                                                index += 1;
                                            }
                                        }
                                        player.cards.remove(index);
                                        self.game_status.last_throw_card = Some(card);
                                        exit_loop = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            let mut guard = queue_arc_after.write().unwrap();
            guard.clear();
        }
    }

    fn have_card(&self, player_number: u8, card: &PMJCard, do_not_care_id: bool) -> bool {
        if do_not_care_id {
            for i in self
                .players
                .get(player_number as usize)
                .unwrap()
                .cards
                .iter()
            {
                if i.card_number == card.card_number && i.card_type == card.card_type {
                    return true;
                }
            }
            return false;
        } else {
            self.players
                .get(player_number as usize)
                .unwrap()
                .cards
                .contains(card)
        }
    }

    fn handle_game_round_wait(&self, player_number: u8) {
        let duration = std::time::Duration::from_secs(1);
        let mut is_need_after_throw: bool = false;
        {
            let queue_arc_wait = sync::Arc::clone(&self.msg_queue_wait);
            let mut exit_loop: bool = false;
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_wait.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if !(action.player_number == player_number) {
                            is_need_after_throw = true;
                            match action.action {
                                //FIXME
                                GameActionWaitRound::Eat(card) => {
                                    if self.game_status.last_throw_card.is_some()
                                        && card == self.game_status.last_throw_card.unwrap()
                                        && card.card_number > 0
                                        && ((card.card_number > 1
                                            && card.card_number < 9
                                            && self.have_card(
                                                action.player_number,
                                                &PMJCard {
                                                    card_type: card.card_type,
                                                    card_number: card.card_number - 1,
                                                    card_id: 0,
                                                },
                                                true,
                                            )
                                            && self.have_card(
                                                action.player_number,
                                                &PMJCard {
                                                    card_type: card.card_type,
                                                    card_number: card.card_number + 1,
                                                    card_id: 0,
                                                },
                                                true,
                                            ))
                                            || (card.card_number == 1
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number + 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number + 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                ))
                                            || (card.card_number == 9
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number - 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number - 2,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )))
                                    {
                                        todo!("Not Finish") //TODO
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            {
                let mut guard = queue_arc_wait.write().unwrap();
                guard.clear();
            }
        }
        //
        if is_need_after_throw {
            let mut exit_loop: bool = false;
            let queue_arc_after = sync::Arc::clone(&self.msg_queue_after);
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_after.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionAfter::Throw(card) => {
                                    if self
                                        .players
                                        .get(player_number as usize)
                                        .unwrap()
                                        .cards
                                        .contains(&card)
                                    {
                                        let player =
                                            self.players.get_mut(player_number as usize).unwrap();
                                        let mut index: usize = 0;
                                        //FIXME:不太可能出現的邏輯問題
                                        for i in player.cards.iter() {
                                            if i == &card {
                                                break;
                                            } else {
                                                index += 1;
                                            }
                                        }
                                        player.cards.remove(index);
                                        exit_loop = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            let mut guard = queue_arc_after.write().unwrap();
            guard.clear();
        }
    }

    fn get_one_unuse_card(&mut self, player_number: usize) {
        let mut rng = rand::rng();
        let player = self.players.get_mut(player_number).unwrap();
        for _ in 0..4 {
            let card = self.unuse_cards.choose(&mut rng).unwrap().clone();
            let mut index = 0;
            for unuse_card in self.unuse_cards.iter() {
                if unuse_card.clone() == card.clone() {
                    break;
                } else {
                    index += 1;
                }
            }
            self.unuse_cards.remove(index);
            player.cards.push(card);
        }
    }
}
