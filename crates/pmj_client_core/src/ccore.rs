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

//! pmj_client_core::client

use std::{net, sync, thread, time};

use serde_json;
use tracing::{debug, error, info, trace, warn};
use tungstenite;
use url;

use pmj_gamemodes;
use pmj_shared;

use crate::error;

/// WebsocketConnection
type WsConn = tungstenite::WebSocket<net::TcpStream>;

#[derive(Debug)]
struct ClientTask {
    kind: CTaskKinds,
    handle: thread::JoinHandle<Result<CTaskResult, error::CCError>>,
}
#[derive(Debug, Clone)]
enum CTaskKinds {
    SendFirstMsgReq,
    ReadFirstMsgResp,
    PingPong,
    ReadWsMsg,
    ThrowCard(pmj_gamemodes::v2_better::shared::PMJCard),
}
#[derive(Debug, Clone)]
struct CTaskResult {
    rfirstmsgresp_player_id: Option<u8>,
    rfirstmsgresp_gamemode: Option<pmj_shared::shared::GameModes>,
    read_ws_msg_v2: Option<pmj_gamemodes::v2_better::shared::ServerMessage>,
}

impl Default for CTaskResult {
    fn default() -> Self {
        Self {
            rfirstmsgresp_player_id: None,
            rfirstmsgresp_gamemode: None,
            read_ws_msg_v2: None,
        }
    }
}

#[derive(Debug, Clone)]
enum GMState {
    HomePage,
    V2Better(V2BetterState),
}

#[derive(Debug)]
pub struct ClientCore {
    ws: sync::Arc<sync::Mutex<WsConn>>,
    tasks: Vec<ClientTask>,
    gamemode_state: GMState,
}

#[derive(Debug, Clone)]
pub struct V2BetterState {
    player_id: u8,
    cards: Vec<pmj_gamemodes::v2_better::shared::PMJCard>,
    player_turn: Option<u8>,
    game_events: Vec<(u64, V2BetterEvents)>,
    room_msgs: Vec<(u64, pmj_gamemodes::v2_better::shared::ServerRoomMsg)>,
}

#[derive(Debug, Clone)]
pub enum V2BetterEvents {
    GameStart,
    GameFinish,
    ChangeTurn(u8),
    PlayerAction((u8, pmj_gamemodes::v2_better::shared::GameActions)),
    YouGetCard(pmj_gamemodes::v2_better::shared::PMJCard),
    YouHandCardChange(Vec<pmj_gamemodes::v2_better::shared::PMJCard>),
}

pub enum PlayerCtrl {
    NoCtrl,
    ThrowCard,
}

impl ClientCore {
    pub fn current_ctrl(&self) -> PlayerCtrl {
        match self.gamemode_state {
            GMState::HomePage => {
                PlayerCtrl::NoCtrl
            }
            GMState::V2Better(ref state) => {
                let (_event_num, event)=state.game_events.get(state.game_events.len() - 1).unwrap();
                match event {
                    V2BetterEvents::YouGetCard(_) => {PlayerCtrl::ThrowCard}
                    _ => {PlayerCtrl::NoCtrl}
                }
            }
        }
    }
    pub fn connect(server_url: String) -> Result<Self, error::CCError> {
        let uri: url::Url = url::Url::parse(&server_url).unwrap();
        let host = uri.host_str().unwrap();
        let port = uri.port().unwrap();
        let tcp_stream = net::TcpStream::connect((host, port)).unwrap();
        match tcp_stream.set_nodelay(true) {
            Ok(_) => {}
            Err(e) => {
                warn!("set_nodelay: {}", e);
            }
        }
        match tcp_stream.set_read_timeout(Some(time::Duration::from_secs(4))) {
            Ok(_) => {}
            Err(e) => {
                warn!("set_read_timeout: {}", e);
            }
        }
        match tcp_stream.set_write_timeout(Some(time::Duration::from_secs(5))) {
            Ok(_) => {}
            Err(e) => {
                warn!("set_write_timeout: {}", e);
            }
        }
        match tungstenite::client::client(uri.to_string(), tcp_stream) {
            Ok((orig_ws, _resp)) => {
                debug!("Websocket 連線成功。");
                let mut ccore = Self {
                    ws: sync::Arc::new(sync::Mutex::new(orig_ws)),
                    tasks: Vec::new(),
                    gamemode_state: GMState::HomePage,
                };
                ccore.send_first_msg_req();
                Result::Ok(ccore)
            }
            Err(e) => {
                warn!("ws connect error: {}", e);
                Result::Err(error::CCError {
                    kind: error::CCErrKinds::HandShakeError,
                })
            }
        }
    }

    fn send_first_msg_req(&mut self) {
        let thread_ws = self.ws.clone();
        let handle = thread::spawn(move || {
            trace!("正在嘗試傳送初連接訊息");
            let req_text = serde_json::to_string(&pmj_shared::shared::ClientConnectRequestType {
                app_name: String::from("positive_mahjong"),
                client: String::from("pmj_client"),
            })
            .unwrap();
            match thread_ws.lock() {
                Ok(mut guard) => {
                    trace!("ws -> get guard");
                    match guard.send(tungstenite::Message::Text(req_text.into())) {
                        Ok(_) => {
                            drop(guard);
                            debug!("已傳送初連結訊息，等待伺服器回應...");
                            Result::Ok(CTaskResult::default())
                        }
                        Err(e) => {
                            drop(guard);
                            warn!("error: {}", e);
                            Result::Err(error::CCError {
                                kind: error::CCErrKinds::Other,
                            })
                        }
                    }
                }
                Err(e) => {
                    warn!("First msg: get guard error: {}", e);
                    Result::Err(error::CCError {
                        kind: error::CCErrKinds::Other,
                    })
                }
            }
        });
        self.tasks.push(ClientTask {
            kind: CTaskKinds::SendFirstMsgReq,
            handle: handle,
        });
    }

    fn read_first_msg_resp(&mut self) {
        let thread_ws = self.ws.clone();
        let handle = thread::spawn(move || {
            loop {
                match thread_ws.lock() {
                    Ok(mut guard) => {
                        match guard.read() {
                            Ok(msg) => {
                                drop(guard);
                                match msg {
                                    tungstenite::Message::Text(text) => {
                                        match serde_json::from_str::<
                                            pmj_shared::shared::ServerConnectResponceType,
                                        >(
                                            &text.to_string()
                                        ) {
                                            Ok(tmsg) => {
                                                if !tmsg.too_many_player {
                                                    return Result::Ok(CTaskResult {
                                                        rfirstmsgresp_player_id: tmsg.player_id,
                                                        rfirstmsgresp_gamemode: Some(tmsg.gamemode),
                                                        ..Default::default()
                                                    });
                                                } else {
                                                    return Result::Err(error::CCError {
                                                        kind: error::CCErrKinds::Other,
                                                    });
                                                }
                                            }
                                            Err(e) => {
                                                error!("read_first_msg_resp: {}", e);
                                                return Result::Err(error::CCError {
                                                    kind: error::CCErrKinds::Other,
                                                });
                                            }
                                        }
                                    }
                                    tungstenite::Message::Ping(_) => {}
                                    tungstenite::Message::Pong(_) => {}
                                    _ => {
                                        //TODO: Binary Support
                                        return Result::Err(error::CCError {
                                            kind: error::CCErrKinds::Other,
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                drop(guard);
                                error!("read_first_msg_resp: {}", e);
                                return Result::Err(error::CCError {
                                    kind: error::CCErrKinds::Other,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        error!("read_first_msg_resp: {}", e);
                        return Result::Err(error::CCError {
                            kind: error::CCErrKinds::Other,
                        });
                    }
                }
            }
        });
        self.tasks.push(ClientTask {
            kind: CTaskKinds::ReadFirstMsgResp,
            handle: handle,
        });
    }

    fn ping_pong_thread(&mut self) {
        let thread_ws = self.ws.clone();
        let handle = thread::spawn(move || {
            loop {
                match thread_ws.lock() {
                    Ok(mut guard) => {
                        match guard.send(tungstenite::Message::Ping(tungstenite::Bytes::new())) {
                            Ok(_) => {
                                drop(guard);
                                thread::sleep(time::Duration::from_secs(3));
                            }
                            Err(e) => {
                                drop(guard);
                                error!("ping: {}", e);
                                thread::sleep(time::Duration::from_secs(1));
                                return Result::Err(error::CCError {
                                    kind: error::CCErrKinds::Other,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        error!("ping: {}", e);
                        thread::sleep(time::Duration::from_secs(1));
                        return Result::Err(error::CCError {
                            kind: error::CCErrKinds::Other,
                        });
                    }
                }
            }
        });
        self.tasks.push(ClientTask {
            kind: CTaskKinds::PingPong,
            handle: handle,
        });
    }

    pub fn throw_card(&mut self, card: pmj_gamemodes::v2_better::shared::PMJCard) {
        let thread_ws = self.ws.clone();
        let thread_card = card.clone();
        let handle = thread::spawn(move || {
            let req_text =
                serde_json::to_string(&pmj_gamemodes::v2_better::shared::ClientMessage::GameMsg(
                    pmj_gamemodes::v2_better::shared::ClientGameMsg::ThrowCard(thread_card),
                ))
                .unwrap();
            match thread_ws.lock() {
                Ok(mut guard) => match guard.send(tungstenite::Message::Text(req_text.into())) {
                    Ok(_) => {
                        drop(guard);
                        Result::Ok(CTaskResult::default())
                    }
                    Err(e) => {
                        drop(guard);
                        warn!("error: {}", e);
                        Result::Err(error::CCError {
                            kind: error::CCErrKinds::Other,
                        })
                    }
                },
                Err(e) => {
                    warn!("throw_card: {}", e);
                    Result::Err(error::CCError {
                        kind: error::CCErrKinds::Other,
                    })
                }
            }
        });
        self.tasks.push(ClientTask {
            kind: CTaskKinds::ThrowCard(card),
            handle: handle,
        });
    }

    pub fn process_task(&mut self) {
        let mut task_index = 0;
        loop {
            if task_index >= self.tasks.len() {
                break;
            } else {
                match self.tasks.get(task_index) {
                    Some(task_ref) => {
                        if task_ref.handle.is_finished() {
                            let task = self.tasks.remove(task_index);
                            match task.handle.join() {
                                Ok(task_result) => match task_result {
                                    Ok(ctr) => match task.kind {
                                        CTaskKinds::ThrowCard(card) => {
                                            info!("threw {} sucessful.", card);
                                        }
                                        CTaskKinds::ReadWsMsg => {
                                            let server_msg = ctr.read_ws_msg_v2.unwrap();
                                            info!("server_msg: {:?}", server_msg);
                                            match server_msg {
                                                        pmj_gamemodes::v2_better::shared::ServerMessage::RoomMsg(room_msg) => {
                                                            match self.gamemode_state {
                                                                GMState::V2Better(ref mut state_v2) => {
                                                                    state_v2.room_msgs.push((
                                                                        (state_v2.room_msgs.len() as u64) +1,
                                                                        room_msg
                                                                    ));
                                                                }
                                                                GMState::HomePage => {
                                                                    error!("????????????");
                                                                    panic!("????????????");
                                                                }
                                                            }
                                                        }
                                                        pmj_gamemodes::v2_better::shared::ServerMessage::GameMsg(game_msg) =>{
                                                            match self.gamemode_state {
                                                                GMState::V2Better(ref mut state_v2) => {
                                                                    match game_msg.msg_type {
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::GameStart => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::GameStart)
                                                                            );
                                                                        }
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::ChangedTurn => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::ChangeTurn(game_msg.info_change_turn.unwrap()))
                                                                            );
                                                                        }
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::Error => { todo!("not support yet")}
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::GameFinish => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::GameFinish)
                                                                            );
                                                                        }
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::GetCard => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::YouGetCard(game_msg.info_get_card.unwrap()))
                                                                            );
                                                                        }
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::HandCardChange => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::YouHandCardChange(game_msg.info_hand_card_change.unwrap()))
                                                                            );
                                                                        }
                                                                        pmj_gamemodes::v2_better::shared::ServerGameMsgKinds::PlayerAction => {
                                                                            state_v2.game_events.push(
                                                                                ((state_v2.game_events.len() as u64) +1,
                                                                                    V2BetterEvents::PlayerAction(game_msg.info_player_action.unwrap()))
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                                GMState::HomePage => {
                                                                    error!("????????????");
                                                                    panic!("????????????");
                                                                }
                                                            }
                                                        }
                                                    }
                                        }
                                        CTaskKinds::SendFirstMsgReq => {
                                            self.read_first_msg_resp();
                                            break;
                                        }
                                        CTaskKinds::PingPong => {
                                            warn!("task ping finished sucessful?");
                                            self.ping_pong_thread();
                                            break;
                                        }
                                        CTaskKinds::ReadFirstMsgResp => {
                                            let player_id = ctr.rfirstmsgresp_player_id.unwrap();
                                            match ctr.rfirstmsgresp_gamemode.unwrap() {
                                                pmj_shared::shared::GameModes::V2Better => {}
                                                _ => {
                                                    todo!("not support yet")
                                                }
                                            }
                                            self.gamemode_state =
                                                GMState::V2Better(V2BetterState {
                                                    player_id: player_id,
                                                    cards: Vec::new(),
                                                    player_turn: None,
                                                    game_events: Vec::new(),
                                                    room_msgs: Vec::new(),
                                                });
                                            self.ping_pong_thread();
                                        }
                                    },
                                    Err(e) => {
                                        error!("task {:?}: {}", task.kind, e);
                                        match task.kind {
                                            CTaskKinds::ThrowCard(card) => {
                                                self.throw_card(card);
                                                break;
                                            }
                                            CTaskKinds::ReadWsMsg => {
                                                self.read_ws_msg();
                                                break;
                                            }
                                            CTaskKinds::SendFirstMsgReq => {
                                                self.send_first_msg_req();
                                                break;
                                            }
                                            CTaskKinds::ReadFirstMsgResp => {
                                                self.read_first_msg_resp();
                                                break;
                                            }
                                            CTaskKinds::PingPong => {
                                                self.ping_pong_thread();
                                                break;
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    error!("task {:?}: {:?}", task.kind, e);
                                    match task.kind {
                                        CTaskKinds::ThrowCard(card) => {
                                            self.throw_card(card);
                                            task_index += 1;
                                        }
                                        CTaskKinds::ReadWsMsg => {
                                            self.read_ws_msg();
                                            task_index += 1;
                                        }
                                        CTaskKinds::SendFirstMsgReq => {
                                            self.send_first_msg_req();
                                            task_index += 1;
                                        }
                                        CTaskKinds::ReadFirstMsgResp => {
                                            self.read_first_msg_resp();
                                            task_index += 1;
                                        }
                                        CTaskKinds::PingPong => {
                                            self.ping_pong_thread();
                                            task_index += 1;
                                        }
                                    }
                                }
                            }
                        } else {
                            task_index += 1;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    fn read_ws_msg(&mut self) {
        let thread_ws = self.ws.clone();
        let handle = thread::spawn(move || {
            loop {
                match thread_ws.lock() {
                    Ok(mut guard) => match guard.read() {
                        Ok(msg) => {
                            drop(guard);
                            match msg {
                                tungstenite::Message::Text(text) => {
                                    match serde_json::from_str::<
                                        pmj_gamemodes::v2_better::shared::ServerMessage,
                                    >(&text.to_string())
                                    {
                                        Ok(tmsg) => {
                                            return Result::Ok(CTaskResult {
                                                read_ws_msg_v2: Some(tmsg),
                                                ..Default::default()
                                            });
                                        }
                                        Err(e) => {
                                            error!("read_ws_msg: {}", e);
                                            return Result::Err(error::CCError {
                                                kind: error::CCErrKinds::Other,
                                            });
                                        }
                                    }
                                }
                                tungstenite::Message::Pong(_) => {}
                                tungstenite::Message::Ping(_) => {}
                                _ => {
                                    error!("unsupport msg kind: {}", msg);
                                    return Result::Err(error::CCError {
                                        kind: error::CCErrKinds::Other,
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            drop(guard);
                            error!("read_ws_msg: {}", e);
                            return Result::Err(error::CCError {
                                kind: error::CCErrKinds::Other,
                            });
                        }
                    },
                    Err(e) => {
                        error!("read_ws_msg: {}", e);
                        return Result::Err(error::CCError {
                            kind: error::CCErrKinds::Other,
                        });
                    }
                }
            }
        });
        self.tasks.push(ClientTask {
            kind: CTaskKinds::ReadWsMsg,
            handle: handle,
        })
    }
}
