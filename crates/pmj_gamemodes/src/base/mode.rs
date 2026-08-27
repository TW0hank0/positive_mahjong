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

use std::{
    self,
    net::{self, TcpListener, TcpStream},
    sync::{self, Arc, RwLock},
    thread, time,
};

use rand::{self, prelude::SliceRandom, seq::IndexedRandom};
use tracing::{debug, error, info, trace, warn};
use tungstenite::{Message, WebSocket, accept_with_config};

use pmj_shared::shared;

use crate::base::shared as shared_base;
use crate::base::{
    self,
    shared::{GameActions, PMJCard, PMJCardFlowerType, PMJCardType, PMJCardWordsType, PMJPlayer},
};

fn write_reply(
    text: String,
    websocket: sync::Arc<sync::RwLock<WebSocket<TcpStream>>>,
) -> Result<(), tungstenite::error::Error> {
    trace!("enter_func {{ text: {} }}", text);
    debug!("準備回覆客戶端...");
    let reply: Message = Message::Text(text.into());
    let write_result: tungstenite::Result<()>;
    loop {
        match websocket.write() {
            Ok(mut guard) => {
                write_result = guard.write(reply.clone());
                let _ = guard.flush();
                drop(guard);
                break;
            }
            Err(e) => {
                warn!("ws.try_write() Err: {}", e);
            }
        };
        thread::sleep(time::Duration::from_millis(500));
    }
    match write_result {
        Ok(_) => {
            info!("成功回覆客戶端。")
        }
        Err(_) => {
            warn!("回覆客戶端失敗！")
        }
    }
    write_result
}

// 處理單一客戶端連線的函式
fn handle_client(
    tcp_stream: TcpStream,
    backend: sync::Arc<sync::RwLock<crate::base::mode::PositiveMahjong>>,
) {
    tcp_stream
        .set_read_timeout(Some(time::Duration::from_secs(8)))
        .ok();
    tcp_stream
        .set_write_timeout(Some(time::Duration::from_secs(10)))
        .ok();
    tcp_stream.set_nodelay(true).ok();
    let client_ip = tcp_stream.peer_addr().unwrap().ip();
    info!("建立連線：{}", client_ip.to_string());
    let websocket: WebSocket<TcpStream> = match accept_with_config(
        tcp_stream,
        Some(tungstenite::protocol::WebSocketConfig::default()),
    ) {
        Ok(ws) => ws,
        Err(e) => {
            warn!("ws握手失敗：{}", e);
            return;
        }
    };
    let ws: sync::Arc<sync::RwLock<WebSocket<TcpStream>>> =
        sync::Arc::new(sync::RwLock::new(websocket));

    info!("客戶端 Websocket 連線成功。");

    // 進入訊息接收迴圈
    'connection: loop {
        let message: tungstenite::Message;
        // 讀取訊息
        'read_msg: loop {
            match ws.try_write() {
                Ok(mut guard) => {
                    if !guard.can_read() {
                        warn!("guard.can_read() = false");
                    } else {
                        debug!("guard.can_read() -> true");
                        //guard.get_mut().set_nonblocking(true).ok();
                        match guard.read() {
                            Ok(msg) => {
                                message = msg;
                                //guard.get_mut().set_nonblocking(false).ok();
                                break 'read_msg;
                            }
                            Err(e) => {
                                warn!("讀取錯誤：{}", e);
                                //guard.get_mut().set_nonblocking(false).ok();
                                drop(guard);
                                thread::sleep(std::time::Duration::from_millis(2569));
                                // 2sec
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("ws.try_write() Err: {}", e);
                    thread::sleep(std::time::Duration::from_secs(1)); // 2sec
                }
            }
        }
        match message {
            Message::Text(text) => {
                trace!("message -> Message::Text => {}", text);
                let value: Result<shared::ClientConnectRequestType, serde_json::Error> =
                    serde_json::from_str(&text);
                match value {
                    Ok(req) => {
                        if req.app_name != "positive_mahjong" {
                            let _reply_result = write_reply(
                                "這是 `positive_mahjong` 的伺服器端！".to_string(),
                                sync::Arc::clone(&ws),
                            );
                        } else {
                            let result_player_id: Option<u8>;
                            'get_player_id: loop {
                                match backend.try_write() {
                                    Ok(mut guard) => {
                                        result_player_id = guard.add_player(client_ip, ws.clone());
                                        break 'get_player_id;
                                    }
                                    Err(e) => {
                                        warn!("backend.try_write() Err: {}", e);
                                        continue 'get_player_id;
                                    }
                                }
                            }
                            let resp = if result_player_id.is_none() {
                                shared::ServerConnectResponceType {
                                    gamemode: shared::GameModes::Base,
                                    player_id: None,
                                    too_many_player: true,
                                }
                            } else {
                                debug!("後端已返回玩家識別碼：{:?}", result_player_id.clone());
                                shared::ServerConnectResponceType {
                                    gamemode: shared::GameModes::Base,
                                    player_id: result_player_id,
                                    too_many_player: false,
                                }
                            };
                            let resp_msg = serde_json::to_string(&resp).unwrap();
                            let _wrist_result = write_reply(resp_msg, ws.clone());
                            info!("已回復客戶端初訊息。");
                            trace!("因為連接並回覆初訊息，將定期發送 Ping。");
                            loop {
                                match ws.try_write() {
                                    Ok(mut guard) => {
                                        match guard.send(tungstenite::Message::Ping(
                                            tungstenite::Bytes::new(),
                                        )) {
                                            Ok(_) => {
                                                trace!("成功發送 Ping。");
                                            }
                                            Err(
                                                tungstenite::Error::AlreadyClosed
                                                | tungstenite::Error::ConnectionClosed,
                                            ) => {
                                                break 'connection;
                                            }
                                            Err(e) => {
                                                trace!("ping process err: {:?}", e);
                                                thread::sleep(time::Duration::from_secs(1));
                                            }
                                        }
                                    }
                                    Err(_e) => {
                                        thread::sleep(time::Duration::from_secs(1));
                                    }
                                }
                                thread::sleep(time::Duration::from_secs(2));
                            }
                        }
                    }
                    Err(e) => {
                        let msg = format!("客戶端請求格式（json）錯誤：{}", e);
                        debug!("{}", msg.clone());
                        let _reply_result = write_reply(msg, sync::Arc::clone(&ws));
                    }
                }
            }
            Message::Binary(_data) => {
                // TODO: msgpack
                debug!("跳過Binary Message!");
            }
            Message::Ping(_) => {
                // 函式庫通常會自動處理 Pong，亦可手動處理
            }
            Message::Pong(_) => {
                // 忽略 Pong
            }
            Message::Close(_) => {
                info!("客戶端請求關閉連線");
                break 'connection;
            }
            Message::Frame(_) => {
                // 忽略原始帧
            }
        }
        thread::sleep(std::time::Duration::from_millis(1536)); //1.5sec
    }
}

pub fn main_base(gui_mode: bool) -> Option<Arc<RwLock<crate::base::mode::PositiveMahjong>>> {
    let backend = sync::Arc::new(sync::RwLock::new(crate::base::mode::PositiveMahjong::new()));
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
        Some(backend)
    } else {
        info!("按 enter 開始遊戲");
        std::io::stdin().read_line(&mut String::new()).ok();
        loop {
            match backend.try_write() {
                Ok(mut guard) => {
                    guard.start_game();
                    break;
                }
                Err(e) => {
                    warn!("backend.try_write() => {:?}", e);
                    thread::sleep(time::Duration::from_millis(500));
                }
            }
        }
        'join_servers: loop {
            match servers.pop() {
                Some(server) => {
                    let thread_result = server.join();
                    debug!("thread_result: {:?}", thread_result);
                }
                None => {
                    break 'join_servers;
                }
            }
        }
        None
    }
}

fn handle_server_base(
    addr: std::net::SocketAddr,
    backend: sync::Arc<sync::RwLock<crate::base::mode::PositiveMahjong>>,
) {
    // 建立 TCP Listener
    let listener: TcpListener = match TcpListener::bind(addr) {
        Ok(i) => {
            info!("已綁定：{}", addr.clone());
            i
        }
        Err(e) => {
            warn!("無法綁定Port：{}", e);
            return;
        }
    };
    let mut thread_handles = Vec::new();
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                let thread_backend = sync::Arc::clone(&backend);
                let handle = std::thread::spawn(move || {
                    handle_client(stream, thread_backend);
                });
                thread_handles.push(handle);
            }
            Err(e) => {
                warn!("連線失敗：{}", e);
            }
        }
    }
    let sleep_dur = std::time::Duration::from_millis(700);
    for handle in thread_handles {
        if !handle.is_finished() {
            thread::sleep(sleep_dur);
        }
    }
}

#[derive(Debug)]
pub struct PositiveMahjong {
    players: Vec<base::shared::PMJPlayer>,
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
                    card_id,
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
                    card_id,
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
                    card_id,
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
                    card_id,
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
            unused_card,
        }
    }

    pub fn get_players_info(&self) -> Vec<PMJPlayer> {
        self.players.clone()
    }

    pub fn is_game_start(&self) -> bool {
        self.is_game_start
    }

    pub fn is_game_finish(&self) -> bool {
        self.is_game_finish
    }

    /// 返回 player_id 或是 None (人數已滿)
    ///
    /// TODO: 用 Result 替換 Option
    pub fn add_player(
        &mut self,
        player_ip_addr: net::IpAddr,
        player_ws: sync::Arc<sync::RwLock<WebSocket<TcpStream>>>,
    ) -> Option<u8> {
        let current_player_count = self.players.len();
        if (current_player_count as u8) < shared_base::MAX_PLAYER_COUNT {
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
            ..Default::default()
        })
        .unwrap();
        for player in self.players.iter() {
            info!(
                "start_game -> 通知遊戲開始：{}. {}",
                player.player_id,
                player.player_ip_addr.to_string()
            );
            let _write_result = write_reply(game_start_msg.clone(), player.player_ws.clone());
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
            info!(
                "start_game -> 通知手牌變動： {}. {}, 卡牌：{:?}",
                player.player_id,
                player.player_ip_addr.to_string(),
                player.player_hand_cards.clone()
            );
            let hand_card_msg = serde_json::to_string(&shared_base::ServerMessageType {
                msg_type: shared_base::ServerMessageTypeKinds::HandCardChange,
                info_hand_card_change: Some(player.player_hand_cards.clone()),
                ..Default::default()
            })
            .unwrap();
            let _write_result = write_reply(hand_card_msg, player.player_ws.clone());
        }
        //
        self.game_loop();
    }

    /// 遊戲旋環
    fn game_loop(&mut self) {
        let mut current_turn_player_id: u8 = 1;
        let mut current_action: GameActions = GameActions::GetCard;
        let players_count = self.players.len() as u8;
        // rng init
        let mut rng = rand::rng();
        self.unused_card.shuffle(&mut rng);
        // main loop
        'game: loop {
            {
                let msg = serde_json::to_string(&shared_base::ServerMessageType {
                    msg_type: shared_base::ServerMessageTypeKinds::ChangedTurn,
                    info_change_turn: Some(current_turn_player_id),
                    ..Default::default()
                })
                .unwrap();
                for player in self.players.iter() {
                    write_reply(msg.clone(), player.player_ws.clone()).ok();
                }
            }
            match current_action {
                GameActions::GetCard => {
                    let player = self
                        .players
                        .get_mut((current_turn_player_id - 1) as usize) // `-1` to match index
                        .unwrap();
                    {
                        'choose_card: loop {
                            let got_card = self.unused_card.choose(&mut rng).unwrap();
                            if got_card.card_type != PMJCardType::Flower {
                                let mut index = 0;
                                'find_index: for i in self.unused_card.iter() {
                                    if i == got_card {
                                        break 'find_index;
                                    } else {
                                        index += 1;
                                    }
                                }
                                let player_card = self.unused_card.remove(index);
                                player.player_hand_cards.push(player_card.clone());
                                let client_msg =
                                    serde_json::to_string(&shared_base::ServerMessageType {
                                        msg_type: shared_base::ServerMessageTypeKinds::GetCard,
                                        info_get_card: Some(player_card),
                                        ..Default::default()
                                    })
                                    .unwrap();
                                write_reply(client_msg, player.player_ws.clone()).ok();
                                break 'choose_card;
                            }
                        }
                    }
                    current_action = GameActions::ThrowCard;
                }
                GameActions::ThrowCard => {
                    let player = self
                        .players
                        .get_mut((current_turn_player_id - 1) as usize)
                        .unwrap();
                    let player_ws = player.player_ws.clone();
                    'get_player_throw: loop {
                        let ws_msg: tungstenite::Message;
                        'guard_read: loop {
                            match player_ws.write() {
                                Ok(mut guard) => match guard.read() {
                                    Ok(i) => {
                                        ws_msg = i;
                                        break 'guard_read;
                                    }
                                    Err(tungstenite::Error::AlreadyClosed) => {
                                        drop(guard);
                                        error!(
                                            "guard.read() => 連線早已關閉（tungstenite::Error::AlreadyClosed）"
                                        );
                                        thread::sleep(time::Duration::from_millis(500));
                                    }
                                    Err(tungstenite::Error::ConnectionClosed) => {
                                        drop(guard);
                                        error!(
                                            "guard.read() => 連線已關閉（tungstenite::Error::ConnectionClosed）"
                                        );
                                        thread::sleep(time::Duration::from_millis(500));
                                    }
                                    Err(tungstenite::Error::Io(io_err)) => match io_err.kind() {
                                        std::io::ErrorKind::TimedOut => {
                                            debug!(
                                                "{:?}",
                                                guard.send(tungstenite::Message::Ping(
                                                    tungstenite::Bytes::new()
                                                ))
                                            );
                                        }
                                        _ => {
                                            drop(guard);
                                            warn!("guard.read(): Err::Io => {:?}", io_err.kind());
                                            thread::sleep(time::Duration::from_millis(500));
                                        }
                                    },
                                    Err(e) => {
                                        drop(guard);
                                        warn!("guard.read(): {}", e);
                                        thread::sleep(time::Duration::from_millis(500));
                                    }
                                },
                                Err(e) => {
                                    error!("err: {}", e);
                                }
                            }
                            thread::sleep(time::Duration::from_millis(500));
                        }
                        match ws_msg {
                            Message::Text(text) => {
                                let msg: base::shared::ClientMessageType =
                                    serde_json::from_str(&text).unwrap();
                                match msg.msg_type {
                                    base::shared::ClientMessageTypeKinds::GameAction => {
                                        match msg.info_game_action.unwrap() {
                                            GameActions::ThrowCard => {
                                                if player
                                                    .player_hand_cards
                                                    .contains(&msg.info_throw_card.clone().unwrap())
                                                {
                                                    let mut card_index: usize = 0;
                                                    'find_index: loop {
                                                        if &msg.info_throw_card.clone().unwrap()
                                                            == player
                                                                .player_hand_cards
                                                                .get(card_index)
                                                                .unwrap()
                                                        {
                                                            break 'find_index;
                                                        } else {
                                                            card_index += 1;
                                                        }
                                                    }
                                                    player.player_hand_cards.remove(card_index);
                                                    let client_msg = serde_json::to_string(&shared_base::ServerMessageType {
                                                    msg_type: shared_base::ServerMessageTypeKinds::HandCardChange,
                                                    info_hand_card_change: Some(player.player_hand_cards.clone()),
                                                    ..Default::default()
                                                })
                                                .unwrap();
                                                    let _write_result = write_reply(
                                                        client_msg,
                                                        player.player_ws.clone(),
                                                    );
                                                    let msg_to_else_player = serde_json::to_string(&shared_base::ServerMessageType{
                                                    msg_type:shared_base::ServerMessageTypeKinds::PlayerAction,
                                                    info_player_action:Some((current_turn_player_id, GameActions::ThrowCard)),
                                                    ..Default::default()
                                                }).unwrap();
                                                    for p in self.players.iter() {
                                                        if p.player_id != current_turn_player_id {
                                                            let _ = write_reply(
                                                                msg_to_else_player.clone(),
                                                                p.player_ws.clone(),
                                                            );
                                                        }
                                                    }
                                                    if current_turn_player_id >= players_count {
                                                        current_turn_player_id = 1;
                                                    } else {
                                                        current_turn_player_id += 1;
                                                    }
                                                    current_action = GameActions::GetCard;
                                                    break 'get_player_throw;
                                                }
                                            }
                                            _ => {
                                                error!("錯誤：客戶端錯誤訊息");
                                                todo!("錯誤處理");
                                            }
                                        }
                                    }
                                }
                            }
                            Message::Ping(_) => {}
                            Message::Pong(_) => {}
                            _ => {
                                error!("錯誤：客戶端錯誤訊息");
                                todo!("錯誤處理");
                            }
                        }
                    }
                }
                _ => {
                    error!("不支援的動作！Action：{:?}", current_action)
                }
            }
        }
    }
}

impl Default for PositiveMahjong {
    fn default() -> Self {
        Self::new()
    }
}
