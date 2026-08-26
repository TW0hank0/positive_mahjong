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

//! V2Better 伺服器

use std::{
    self,
    net::{self, TcpListener, TcpStream},
    sync::{self, Arc, RwLock},
    thread, time,
};

use crossbeam;
use rand::{self, prelude::SliceRandom, seq::IndexedRandom};
use tracing::{debug, error, info, trace, warn};
use tungstenite::{Message, WebSocket, accept_with_config};
use url;

use pmj_shared::shared;

use crate::v2_better::{
    self, shared::{self as mode_shared, GameActions, PMJCard, PMJCardFlowerType, PMJCardType, PMJCardWordsType, PMJPlayer, PlayerGameActions},
};

#[derive(Debug)]
pub struct MsgMgrThreadResult {
    pub is_error: bool,
}
#[derive(Debug, Clone)]
pub struct MsgMgrTaskmsg {
    pub msg_kind: MsgMgrTaskKinds,
    /// 玩家id
    pub kind_read: Option<u8>,
    pub kind_write: Option<(u8, String)>,
    pub kind_add_player: Option<Vec<PMJPlayer>>,
}
impl Default for MsgMgrTaskmsg {
    fn default() -> Self {
        Self {
            msg_kind: MsgMgrTaskKinds::Ping,
            kind_read: None,
            kind_write: None,
            kind_add_player: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MsgMgrTaskKinds {
    Read,
    Write,
    Ping,
    AddPlayer,
}
#[derive(Debug)]
pub struct MsgMgrTaskresult {
    pub msg_kind: MsgMgrTaskKinds,
    pub kind_read: Option<String>,
}
impl Default for MsgMgrTaskresult {
    fn default() -> Self {
        Self {
            msg_kind: MsgMgrTaskKinds::Write,
            kind_read: None,
        }
    }
}
/// TODO: Read Msg Cache Buf
#[derive(Debug)]
pub struct MessageMgr {
    process_thread: thread::JoinHandle<MsgMgrThreadResult>,
    pub players: Vec<PMJPlayer>,
    task_sender:
        crossbeam::channel::Sender<(MsgMgrTaskmsg, crossbeam::channel::Sender<MsgMgrTaskresult>)>,
    tasks: Vec<(
        u64,
        MsgMgrTaskmsg,
        crossbeam::channel::Receiver<MsgMgrTaskresult>,
    )>,
    last_task_id: u64,
}
impl MessageMgr {
    pub fn new(players: Vec<PMJPlayer>) -> Self {
        let thread_players = players.clone();
        let (handle, task_sender) = MessageMgr::spawn_thread(thread_players);
        Self {
            process_thread: handle,
            players,
            task_sender: task_sender,
            tasks: Vec::new(),
            last_task_id: 0,
        }
    }

    pub fn get_task_result(&mut self, task_id: &u64) -> MsgMgrTaskresult {
        let mut task_index = 0;
        loop {
            let (tid, _tmsg, tresult) = self.tasks.get(task_index).unwrap();
            if tid == task_id {
                return tresult.recv().unwrap();
            } else {
                task_index += 1;
            }
        }
    }

    fn task_new(&mut self, task: MsgMgrTaskmsg) -> u64 {
        let (r_send, r_resv) = crossbeam::channel::bounded(2);
        self.last_task_id += 1;
        self.tasks
            .push((self.last_task_id.clone(), task.clone(), r_resv));
        let sender = self.task_sender.clone();
        sender.send((task, r_send)).ok();
        drop(sender);
        self.last_task_id.clone()
    }

    pub fn is_task_finish(&mut self, task_id: &u64) -> bool {
        let mut task_index = 0;
        loop {
            let (tid, _tmsg, tresult) = self.tasks.get(task_index).unwrap();
            if tid == task_id {
                return !tresult.is_empty();
            } else {
                task_index += 1;
            }
        }
    }

    pub fn task_add_player(&mut self, players: Vec<PMJPlayer>) -> u64 {
        self.players = players.clone();
        self.task_new(MsgMgrTaskmsg {
            msg_kind: MsgMgrTaskKinds::AddPlayer,
            kind_add_player: Some(players),
            ..Default::default()
        })
    }

    pub fn task_read(&mut self, pid:u8) -> u64 {
        self.task_new(MsgMgrTaskmsg { msg_kind: MsgMgrTaskKinds::Read, kind_read: Some(pid),..Default::default() })
    }

    pub fn task_ping(&mut self) -> u64 {
        self.task_new(MsgMgrTaskmsg {
            msg_kind: MsgMgrTaskKinds::Ping,
            ..Default::default()
        })
    }

    fn spawn_thread(
        tplayers: Vec<PMJPlayer>,
    ) -> (
        thread::JoinHandle<MsgMgrThreadResult>,
        crossbeam::channel::Sender<(MsgMgrTaskmsg, crossbeam::channel::Sender<MsgMgrTaskresult>)>,
    ) {
        let (task_sender, task_receiver) = crossbeam::channel::unbounded::<(
            MsgMgrTaskmsg,
            crossbeam::channel::Sender<MsgMgrTaskresult>,
        )>();
        let mut players = tplayers;
        let handle = thread::spawn(move || {
            loop {
                match task_receiver.recv() {
                    Ok((task, result_sender)) => match task.msg_kind.clone() {
                        MsgMgrTaskKinds::Write => {
                            let (task_player, task_content) = task.kind_write.unwrap();
                            loop {
                                let mut index = 0;
                                let p = players.get(index).unwrap();
                                if p.player_id == task_player {
                                    let ws = p.player_ws.clone();
                                    loop {
                                        match ws.write() {
                                            Ok(mut guard) => {
                                                match guard
                                                    .send(tungstenite::Message::text(&task_content))
                                                {
                                                    Ok(_) => {
                                                        result_sender
                                                            .send(MsgMgrTaskresult {
                                                                msg_kind: task.msg_kind,
                                                                kind_read: None,
                                                            })
                                                            .ok();
                                                        return MsgMgrThreadResult {
                                                            is_error: false,
                                                        };
                                                    }
                                                    Err(e) => {
                                                        warn!("msgmgr: {}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                warn!("msgmgr: {}", e);
                                            }
                                        }
                                        thread::sleep(time::Duration::from_secs(1));
                                    }
                                } else {
                                    index += 1;
                                }
                            }
                        }
                        MsgMgrTaskKinds::AddPlayer => {
                            players = task.kind_add_player.unwrap();
                            return MsgMgrThreadResult { is_error: false };
                        }
                        MsgMgrTaskKinds::Ping => {}
                        MsgMgrTaskKinds::Read => {
                            let task_player = task.kind_read.unwrap();
                            loop {
                                let mut index = 0;
                                let p = players.get(index).unwrap();
                                if p.player_id == task_player {
                                    let ws = p.player_ws.clone();
                                    loop {
                                        match ws.write() {
                                            Ok(mut guard) => match guard.read() {
                                                Ok(msg) => match msg {
                                                    tungstenite::Message::Text(text) => {
                                                        result_sender
                                                            .send(MsgMgrTaskresult {
                                                                msg_kind: task.msg_kind,
                                                                kind_read: Some(text.to_string()),
                                                            })
                                                            .ok();
                                                        return MsgMgrThreadResult {
                                                            is_error: false,
                                                        };
                                                    }
                                                    tungstenite::Message::Ping(_) => {}
                                                    tungstenite::Message::Pong(_) => {}
                                                    _ => {
                                                        drop(guard);
                                                        warn!("unmatched message: {:?}", msg);
                                                    }
                                                },
                                                Err(e) => {
                                                    warn!("msgmgr: {}", e);
                                                }
                                            },
                                            Err(e) => {
                                                warn!("msgmgr: {}", e);
                                            }
                                        }
                                        thread::sleep(time::Duration::from_secs(1));
                                    }
                                } else {
                                    index += 1;
                                }
                            }
                        }
                    },
                    Err(_) => {
                        return MsgMgrThreadResult { is_error: true };
                    }
                }
            }
        });
        (handle, task_sender)
    }
}

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
fn handle_client(tcp_stream: TcpStream, backend: sync::Arc<sync::RwLock<PositiveMahjong>>) {
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
                        if req.app_name != String::from("positive_mahjong") {
                            let _reply_result = write_reply(
                                format!("這是 `positive_mahjong` 的伺服器端！"),
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
                                    gamemode: shared::GameModes::V2Better,
                                    player_id: None,
                                    too_many_player: true,
                                }
                            } else {
                                debug!("後端已返回玩家識別碼：{:?}", result_player_id.clone());
                                shared::ServerConnectResponceType {
                                    gamemode: shared::GameModes::V2Better,
                                    player_id: result_player_id,
                                    too_many_player: false,
                                }
                            };
                            let resp_msg = serde_json::to_string(&resp).unwrap();
                            let _wrist_result = write_reply(resp_msg, ws.clone());
                            info!("已回復客戶端初訊息。");
                            trace!("因為連接並回覆初訊息，將定期發送 Ping。");
                            loop {
                                match backend.try_write() {
                                    Ok(mut guard) => {
                                        guard.msg_mgr.task_ping();
                                        drop(guard);
                                    }
                                    Err(e) => {
                                        warn!("ping: {}", e);
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

pub fn main_v2_better(return_backend: bool) -> Option<Arc<RwLock<PositiveMahjong>>> {
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
    let server_backend_ipv4 = sync::Arc::clone(&backend);
    servers.push(thread::spawn(move || {
        handle_server(server_addr_ipv4, server_backend_ipv4)
    }));
    let server_backend_ipv6 = sync::Arc::clone(&backend);
    servers.push(thread::spawn(move || {
        handle_server(server_addr_ipv6, server_backend_ipv6)
    }));
    if return_backend {
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

fn handle_server(addr: net::SocketAddr, backend: sync::Arc<sync::RwLock<PositiveMahjong>>) {
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
    players: Vec<PMJPlayer>,
    is_game_start: bool,
    is_game_finish: bool,
    /// 未被 使用/抽取 的牌
    unused_card: Vec<PMJCard>,
    msg_mgr: MessageMgr,
}

impl PositiveMahjong {
    pub fn new() -> Self {
        let mut unused_card: Vec<PMJCard> = Vec::new();
        //初始化`筒`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::Dots(card_number),
                    card_id: card_id,
                });
            }
        }
        //初始化`條`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::Line(card_number),
                    card_id: card_id,
                });
            }
        }
        //初始化`萬`
        for card_id in 1..=4 {
            for card_number in 1..=9 {
                unused_card.push(PMJCard {
                    card_type: PMJCardType::TenThousand(card_number),
                    card_id: card_id,
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
                card_type: PMJCardType::Flower(flower_type),
                card_id: 1,
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
                    card_type: PMJCardType::Words(word_type),
                    card_id: card_id,
                });
            }
        }
        //
        Self {
            players: Vec::new(),
            is_game_finish: false,
            is_game_start: false,
            unused_card: unused_card,
            msg_mgr: MessageMgr::new(Vec::new()),
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
        if (current_player_count as u8) < mode_shared::MAX_PLAYER_COUNT {
            let player_id: u8 = (current_player_count + 1) as u8;
            self.players.push(PMJPlayer {
                player_ip_addr,
                player_id,
                player_ws,
                player_hand_cards: Vec::new(),
                player_used_cards: Vec::new(),
            });
            self.msg_mgr.task_add_player(self.players.clone());
            Some(player_id)
        } else {
            None
        }
    }

    /// 開始遊戲
    pub fn start_game(&mut self) {
        self.is_game_start = true;
        let game_start_msg = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(
            mode_shared::ServerGameMsg::GameStart,
        ))
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
            let hand_card_msg = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::HandCardChange
                    (player.player_hand_cards.clone()))
            )
            .unwrap();
            let _write_result = write_reply(hand_card_msg, player.player_ws.clone());
        }
        //
        self.game_loop();
    }

    /// 遊戲旋環
    fn game_loop(&mut self) {
        let mut current_turn_player_id: u8 = 1;
        let mut current_action: PlayerGameActions = PlayerGameActions::GetCard;
        let mut last_turn_player_id: u8 = 0;
        let mut last_action = mode_shared::PlayerGameActions::GetCard;
        let mut last_action_need_throw: bool = false;
        let players_count = self.players.len() as u8;
        // rng init
        let mut rng = rand::rng();
        self.unused_card.shuffle(&mut rng);
        // main loop
        'game: loop {
            {
                let msg = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::ChangedTurn
                    (current_turn_player_id.clone())))
                .unwrap();
                for player in self.players.iter() {
                    write_reply(msg.clone(), player.player_ws.clone()).ok();
                }
            }
            {
                let mut rtask_ids = Vec::new();
                for p in self.players.iter() {
                    rtask_ids.push((p.player_id, self.msg_mgr.task_read(p.player_id)));
                }
                for (player_id, rtask_id) in rtask_ids.iter() {
                    if self.msg_mgr.is_task_finish(rtask_id) {
                        let tresult = self.msg_mgr.get_task_result(rtask_id).kind_read.unwrap();
                        let msg = serde_json::from_str::<mode_shared::ClientMessage>(&tresult).unwrap();
                        match msg {
                            mode_shared::ClientMessage::GameMsg(cgm) => {
                                match cgm {
                                    mode_shared::ClientGameMsg::Pga(pga) => {
                                        match pga {
                                            PlayerGameActions::GetCard => {
                                                if player_id == &current_turn_player_id {
                                                    current_action = PlayerGameActions::GetCard;
                                                } else {
                                                    // 客戶端錯誤訊息
                                                    todo!("not impl yet!");
                                                }
                                            }
                                            PlayerGameActions::ThrowCard(card) => {

                                            }
                                        }
                                    }
                                }
                            }
                            mode_shared::ClientMessage::RoomMsg(crm) => {
                                todo!("unsupport yet!");
                            }
                        }
                    }
                }
            }
            {
                let msg = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::PlayerAction(current_turn_player_id, current_action)))
                .unwrap();
                for player in self.players.iter() {
                    write_reply(msg.clone(), player.player_ws.clone()).ok();
                }
            }
            match current_action {
                PlayerGameActions::GetCard => {
                    let player = self
                        .players
                        .get_mut((current_turn_player_id - 1) as usize) // `-1` to match index
                        .unwrap();
                    {
                        'choose_card: loop {
                            let got_card = self.unused_card.choose(&mut rng).unwrap();
                            if !got_card.card_type.is_flower() {
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
                                    serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::GetCard
                                            (player_card)))
                                    .unwrap();
                                write_reply(client_msg, player.player_ws.clone()).ok();
                                break 'choose_card;
                            }
                        }
                    }
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
                                let msg: v2_better::shared::ClientGameMsg =
                                    serde_json::from_str(&text).unwrap();
                                match msg{
                                    v2_better::shared::ClientGameMsg::Pga(pga) => match pga {
                                    v2_better::shared::PlayerGameActions::ThrowCard(want_throw_card) => {
                                                if player
                                                    .player_hand_cards
                                                    .contains(&want_throw_card)
                                                {
                                                    let mut card_index: usize = 0;
                                                    'find_index: loop {
                                                        if &want_throw_card
                                                            == player
                                                                .player_hand_cards
                                                                .get(card_index.clone())
                                                                .unwrap()
                                                        {
                                                            break 'find_index;
                                                        } else {
                                                            card_index += 1;
                                                        }
                                                    }
                                                    player.player_hand_cards.remove(card_index);
                                                    let client_msg = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::HandCardChange
                                                    (player.player_hand_cards.clone())
                                                    ))
                                                .unwrap();
                                                    let _write_result = write_reply(
                                                        client_msg,
                                                        player.player_ws.clone(),
                                                    );
                                                    let msg_to_else_player = serde_json::to_string(&mode_shared::ServerMessage::GameMsg(mode_shared::ServerGameMsg::PlayerAction
                                                    (current_turn_player_id, mode_shared::PlayerGameActions::ThrowCard(want_throw_card.clone())))).unwrap();
                                                    for p in self.players.iter() {
                                                        let _ = write_reply(
                                                            msg_to_else_player.clone(),
                                                            p.player_ws.clone(),
                                                        );
                                                    }
                                                    break 'get_player_throw;
                                                }}
                                            _ => {
                                                error!("錯誤：客戶端錯誤訊息");
                                                todo!("錯誤處理");
                                            }
                                        }}
                                    }


                            Message::Ping(_) => {}
                            Message::Pong(_) => {}
                            _ => {
                                error!("錯誤：客戶端錯誤訊息");
                                todo!("錯誤處理");
                            }}}
                        }

                _ => {
                    error!("不支援的動作！Action：{:?}", current_action)
                }
            }
            last_turn_player_id = current_turn_player_id;
            last_action = current_action;
            if current_turn_player_id >= players_count {
                current_turn_player_id = 1;
            } else {
                current_turn_player_id += 1;
            }
        }
    }
}
