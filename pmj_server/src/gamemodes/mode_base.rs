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

use rand;
use rand::{prelude::SliceRandom, seq::IndexedRandom};

use std;
use std::net;
use std::net::{TcpListener, TcpStream};
use std::sync::{self, Arc, RwLock};
use std::thread;
use tungstenite::{Error, Message, WebSocket, accept};

use pmj_shared::gamemodes_shared::shared_base::{
    PMJCard, PMJCardFlowerType, PMJCardType, PMJCardWordsType, PMJPlayer,
};
use pmj_shared::gamemodes_shared::{self, shared_base};
use pmj_shared::shared;

use crate::gamemodes;

fn write_reply(
    text: String,
    websocket: sync::Arc<sync::RwLock<WebSocket<TcpStream>>>,
) -> Result<(), Error> {
    let reply: Message = Message::Text(text.into());
    let write_result: Result<(), Error> = websocket.write().unwrap().write(reply);
    write_result
}

// 處理單一客戶端連線的函式
fn handle_client(
    stream: TcpStream,
    backend: sync::Arc<sync::RwLock<gamemodes::mode_base::PositiveMahjong>>,
) {
    let client_ip = stream.peer_addr().unwrap().ip();
    println!("建立連線：{}", client_ip.clone());
    // 進行 WebSocket 握手，建立 WebSocket 物件
    let websocket: WebSocket<TcpStream> = match accept(stream) {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("握手失敗：{}", e);
            return;
        }
    };
    let ws: sync::Arc<sync::RwLock<WebSocket<TcpStream>>> =
        sync::Arc::new(sync::RwLock::new(websocket));

    println!("客戶端連線成功");

    // 進入訊息接收迴圈
    'connection: loop {
        // 讀取訊息
        let msg: Result<Message, Error> = ws.write().unwrap().read();

        match msg {
            Ok(message) => {
                match message {
                    Message::Text(text) => {
                        let value: Result<shared::ClientConnectRequestType, serde_json::Error> =
                            serde_json::from_str(&text);
                        match value {
                            Ok(req) => {
                                if req.app_name != String::from("positive_mahjong") {
                                    let _reply_result = write_reply(
                                        format!("這是 `positive_mahjong` 的伺服器！"),
                                        sync::Arc::clone(&ws),
                                    );
                                } else {
                                    let mut guard = backend.write().unwrap();
                                    let result_player_id =
                                        guard.add_player(client_ip, sync::Arc::clone(&ws));
                                    let resp = if result_player_id.is_none() {
                                        shared::ServerConnectResponceType {
                                            gamemode: shared::GameModes::Base,
                                            player_id: None,
                                            too_many_player: true,
                                        }
                                    } else {
                                        shared::ServerConnectResponceType {
                                            gamemode: shared::GameModes::Base,
                                            player_id: result_player_id,
                                            too_many_player: false,
                                        }
                                    };
                                    let resp_msg = serde_json::to_string(&resp).unwrap();
                                    let _wrist_result =
                                        write_reply(resp_msg, sync::Arc::clone(&ws));
                                    thread::sleep(std::time::Duration::from_secs(30));
                                }
                            }
                            Err(e) => {
                                let _reply_result =
                                    write_reply(format!("json錯誤：{}", e), sync::Arc::clone(&ws));
                            }
                        }
                    }
                    Message::Binary(_data) => {
                        // TODO: msgpack
                        println!("跳過Binary Message!");
                    }
                    Message::Ping(_) => {
                        // 函式庫通常會自動處理 Pong，亦可手動處理
                    }
                    Message::Pong(_) => {
                        // 忽略 Pong
                    }
                    Message::Close(_) => {
                        println!("客戶端請求關閉連線");
                        break 'connection;
                    }
                    Message::Frame(_) => {
                        // 忽略原始帧
                    }
                }
            }
            Err(e) => {
                eprintln!("讀取錯誤：{}", e);
                break;
            }
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }

    // 關閉連線
    let _close_result: Result<(), Error> = ws.write().unwrap().close(None);
    //println!("連線已終止");
}

pub fn main_base(gui_mode: bool) -> Arc<RwLock<gamemodes::mode_base::PositiveMahjong>> {
    let backend = sync::Arc::new(sync::RwLock::new(
        gamemodes::mode_base::PositiveMahjong::new(),
    ));
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
    let server_backend_ipv4 = sync::Arc::clone(&backend);
    servers.push(thread::spawn(move || {
        handle_server_base(server_addr_ipv4, server_backend_ipv4)
    }));
    let server_backend_ipv6 = sync::Arc::clone(&backend);
    servers.push(thread::spawn(move || {
        handle_server_base(server_addr_ipv6, server_backend_ipv6)
    }));
    if gui_mode {
        backend
    } else {
        for server in servers {
            let _thread_result = server.join();
        }
        backend
    }
}

fn handle_server_base(
    addr: std::net::SocketAddr,
    backend: sync::Arc<sync::RwLock<gamemodes::mode_base::PositiveMahjong>>,
) {
    // 建立 TCP Listener
    let listener: TcpListener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("無法綁定Port：{}", e);
            return;
        }
    };
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                // 為每個連線啟動新執行緒
                // 使用 move 將 stream 所有權移轉至執行緒
                let thread_backend = sync::Arc::clone(&backend);
                let _handle = std::thread::spawn(move || {
                    handle_client(stream, thread_backend);
                });
            }
            Err(e) => {
                eprintln!("連線失敗：{}", e);
            }
        }
    }
}

#[derive(Debug)]
pub struct PositiveMahjong {
    players: Vec<gamemodes_shared::shared_base::PMJPlayer>,
    is_game_start: bool,
    is_game_finish: bool,
    /// 未被 使用/抽取 的牌
    unused_card: Vec<PMJCard>,
}

impl PositiveMahjong {
    pub fn new() -> Self {
        let mut unused_card: Vec<PMJCard> = Vec::new();
        //初始化`筒`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::Dots,
                    card_id: card_id,
                    info_ten_thousand: None,
                    info_line: None,
                    info_dots: Some(card_number),
                    info_flower: None,
                    info_words: None,
                });
            }
        }
        //初始化`條`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::Line,
                    card_id: card_id,
                    info_ten_thousand: None,
                    info_line: Some(card_number),
                    info_dots: None,
                    info_flower: None,
                    info_words: None,
                });
            }
        }
        //初始化`萬`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::TenThousand,
                    card_id: card_id,
                    info_ten_thousand: Some(card_number),
                    info_line: None,
                    info_dots: None,
                    info_flower: None,
                    info_words: None,
                });
            }
        }
        //初始化`花`
        for flower_type in [
            PMJCardFlowerType::Bamboo,
            PMJCardFlowerType::Chrysanthemum,
            PMJCardFlowerType::Fall,
            PMJCardFlowerType::Orchid,
            PMJCardFlowerType::Plum,
            PMJCardFlowerType::Spring,
            PMJCardFlowerType::Summer,
            PMJCardFlowerType::Winter,
        ] {
            unused_card.push(PMJCard {
                card_type: PMJCardType::Flower,
                card_id: 1,
                info_ten_thousand: None,
                info_line: None,
                info_dots: None,
                info_flower: Some(flower_type),
                info_words: None,
            });
        }
        //初始化`字`
        for card_id in 1..=4 {
            for word_type in [
                PMJCardWordsType::East,
                PMJCardWordsType::GreenDragon,
                PMJCardWordsType::North,
                PMJCardWordsType::RedDragon,
                PMJCardWordsType::South,
                PMJCardWordsType::West,
                PMJCardWordsType::WhiteDragon,
            ] {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::Words,
                    card_id: card_id,
                    info_ten_thousand: None,
                    info_line: None,
                    info_dots: None,
                    info_flower: None,
                    info_words: Some(word_type),
                });
            }
        }
        //
        Self {
            players: Vec::new(),
            is_game_finish: false,
            is_game_start: false,
            unused_card: unused_card,
        }
    }

    pub fn is_game_start(&self) -> bool {
        self.is_game_start
    }

    pub fn is_game_finish(&self) -> bool {
        self.is_game_finish
    }

    /// 返回player_id或是 None(人數已滿)
    pub fn add_player(
        &mut self,
        player_ip_addr: net::IpAddr,
        player_ws: sync::Arc<sync::RwLock<WebSocket<TcpStream>>>,
    ) -> Option<u8> {
        let current_player_count = self.players.len();
        if current_player_count < 4 {
            let player_id: u8 = (current_player_count + 1) as u8;
            self.players.push(PMJPlayer {
                player_ip_addr,
                player_id,
                player_ws,
                player_hand_cards: Vec::new(),
                player_used_cards: Vec::new(),
            });
            Some(player_id)
        } else {
            None
        }
    }

    /// 開始遊戲
    pub fn start_game(&mut self) {
        self.is_game_start = true;
        let game_start_msg = serde_json::to_string(&shared_base::ServerMessageType {
            msg_type: shared_base::ServerMessageTypeKinds::GameStart,
        })
        .unwrap();
        for player in self.players.iter() {
            let _write_result =
                write_reply(game_start_msg.clone(), sync::Arc::clone(&player.player_ws));
        }
        // rng init
        let mut rng = rand::rng();
        self.unused_card.shuffle(&mut rng);
        // 四次
        for _ in 0..4 {
            for player in self.players.iter_mut() {
                // 一次4張
                for _ in 0..4 {
                    let card = self.unused_card.choose(&mut rng).unwrap();
                    let mut index = 0;
                    'find_index: for i in self.unused_card.iter() {
                        if i == card {
                            break 'find_index;
                        } else {
                            index += 1;
                        }
                    }
                    let player_card = self.unused_card.remove(index);
                    player.player_hand_cards.push(player_card);
                }
            }
        }
        // 通知手牌變動
        for player in self.players.iter() {
            let hand_card_msg = serde_json::to_string(&shared_base::ServerMessageType {
                msg_type: shared_base::ServerMessageTypeKinds::HandCardChange(
                    player.player_hand_cards.clone(),
                ),
            })
            .unwrap();
            let _write_result = write_reply(hand_card_msg, sync::Arc::clone(&player.player_ws));
        }
        //
        self.game_loop();
    }

    /// 遊戲旋環
    fn game_loop(&self) {
        let mut current_turn_player_id = 1;
        let players_count = self.players.len() as u8;
        loop {
            if current_turn_player_id > players_count {
                current_turn_player_id = 1;
            }
            let turn_msg = serde_json::to_string(&shared_base::ServerMessageType {
                msg_type: shared_base::ServerMessageTypeKinds::ChangedTurn(current_turn_player_id),
            })
            .unwrap();
            for player in self.players.iter() {
                let _write_result =
                    write_reply(turn_msg.clone(), sync::Arc::clone(&player.player_ws));
            }
            current_turn_player_id += 1;
        }
    }
}
