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
use std::fs;
use std::sync;
use tiny_http;

use std::net::{TcpListener, TcpStream};
use std::thread;
use tungstenite::protocol::Role;
use tungstenite::{Error, Message, WebSocket, accept};

use pmj_shared::gamemodes_shared;
use pmj_shared::shared;

use crate::gamemodes;

const CURRENT_GAMEMODE: shared::GameModes = shared::GameModes::V1Simple;

pub fn main() {
    println!("ipv4: {}", local_ip_address::local_ip().unwrap());
    println!("ipv6: {}", local_ip_address::local_ipv6().unwrap());
    //
    let config: shared::PMJConfig = if fs::exists(shared::SERVER_CONFIG_FILE_NAME).unwrap_or(false)
    {
        let config_str = fs::read_to_string(shared::SERVER_CONFIG_FILE_NAME).unwrap();
        serde_json::from_str(&config_str).unwrap()
    } else {
        let default_config = shared::PMJConfig::default();
        fs::write(
            shared::SERVER_CONFIG_FILE_NAME,
            serde_json::to_string_pretty(&default_config).unwrap(),
        )
        .ok();
        default_config
    };
    match config.gamemode {
        shared::GameModes::Base => {
            println!("還未支援");
        }
        shared::GameModes::V1Simple => {
            main_v1_simple();
        }
        shared::GameModes::V2Better => {
            println!("還未支援");
        }
    }
}

fn main_v1_simple() {
    let backend = sync::Arc::new(sync::RwLock::new(
        gamemodes::modev1_simple::PositiveMahjong::new(),
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
    servers.push(handle_server_v1_simple(
        server_addr_ipv4,
        sync::Arc::clone(&backend),
    ));
    servers.push(handle_server_v1_simple(
        server_addr_ipv6,
        sync::Arc::clone(&backend),
    ));
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

fn handle_server_v1_simple(
    addr: std::net::SocketAddr,
    backend: sync::Arc<sync::RwLock<gamemodes::modev1_simple::PositiveMahjong>>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let server = tiny_http::Server::http(addr).unwrap();
        for request in server.incoming_requests() {
            let arc_backend = sync::Arc::clone(&backend);
            std::thread::spawn(move || {
                handle_request_v1_simple(request, arc_backend);
            });
        }
    })
}

fn handle_server_request_v1_simple(
    mut request: tiny_http::Request,
    backend: sync::Arc<sync::RwLock<gamemodes::modev1_simple::PositiveMahjong>>,
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
                if content_data.is_test_connection {
                    let response_data = shared::ServerResponseDataType {
                        data_test_connection: Some(shared::ServerResponseDataTestConnectionType {
                            msg: String::new(),
                        }),
                        ..Default::default()
                    };
                    let response = tiny_http::Response::from_string(
                        serde_json::to_string(&shared::ServerResponseType {
                            app: content_data.app,
                            data: response_data, //response_data,
                            msg: String::new(),
                            is_error: false,
                            gamemode: Some(CURRENT_GAMEMODE),
                        })
                        .unwrap(),
                    );
                    match request.respond(response) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                } else {
                    match content_data.data.req_action_type {
                        shared::ActionType::AddPlayer => {
                            let mut guard = backend.write().unwrap();
                            match guard.add_player(request.remote_addr().unwrap().clone()) {
                                gamemodes_shared::sharedv1_simple::Either::Left(e) => {
                                    let response_data = shared::ServerResponseDataType {
                                        data_add_player: None,
                                        data_type: content_data.data.req_action_type,
                                        ..Default::default()
                                    };
                                    let response = tiny_http::Response::from_string(
                                        serde_json::to_string(&shared::ServerResponseType {
                                            app: content_data.app,
                                            data: response_data,
                                            msg: e,
                                            is_error: true,
                                            gamemode: Some(CURRENT_GAMEMODE),
                                        })
                                        .unwrap(),
                                    );
                                    request.respond(response).ok();
                                }
                                gamemodes_shared::sharedv1_simple::Either::Right(number) => {
                                    let response_data = shared::ServerResponseDataType {
                                        data_add_player: Some(
                                            shared::ServerResponseDataAddPlayerType {
                                                number: number,
                                            },
                                        ),
                                        data_type: content_data.data.req_action_type,
                                        ..Default::default()
                                    };
                                    let response = tiny_http::Response::from_string(
                                        serde_json::to_string(&shared::ServerResponseType {
                                            app: content_data.app,
                                            data: response_data,
                                            msg: String::new(),
                                            is_error: false,
                                            gamemode: Some(CURRENT_GAMEMODE),
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
                                    gamemode: Some(CURRENT_GAMEMODE),
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
                                gamemodes_shared::sharedv1_simple::Either::Left(e) => {
                                    let response_data = shared::ServerResponseDataType {
                                        data_add_player: None,
                                        data_type: content_data.data.req_action_type,
                                        ..Default::default()
                                    };
                                    let response = tiny_http::Response::from_string(
                                        serde_json::to_string(&shared::ServerResponseType {
                                            app: content_data.app,
                                            data: response_data,
                                            msg: e,
                                            is_error: true,
                                            gamemode: Some(CURRENT_GAMEMODE),
                                        })
                                        .unwrap(),
                                    );
                                    request.respond(response).ok();
                                }
                                gamemodes_shared::sharedv1_simple::Either::Right(_) => {
                                    let response_data = shared::ServerResponseDataType {
                                        data_type: content_data.data.req_action_type,
                                        ..Default::default()
                                    };
                                    let response = tiny_http::Response::from_string(
                                        serde_json::to_string(&shared::ServerResponseType {
                                            app: content_data.app,
                                            data: response_data,
                                            msg: String::new(),
                                            is_error: false,
                                            gamemode: Some(CURRENT_GAMEMODE),
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
                                        data_type: content_data.data.req_action_type,
                                        data_is_start: Some(
                                            shared::ServerResponseDataIsStartType {
                                                is_start: is_start,
                                            },
                                        ),
                                        ..Default::default()
                                    },
                                    msg: String::new(),
                                    is_error: false,
                                    gamemode: Some(CURRENT_GAMEMODE),
                                })
                                .unwrap(),
                            );
                            request.respond(response).ok();
                        }
                        _ => {
                            todo!("未完成")
                        }
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

fn handle_server_base(
    addr: std::net::SocketAddr,
    backend: sync::Arc<sync::RwLock<gamemodes::mode_base::PositiveMahjong>>,
) {
    // 建立 TCP 監聽器
    let listener: TcpListener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("無法綁定埠號：{}", e);
            return;
        }
    };

    //println!("伺服器已啟動，監聽 {}", addr);

    // 接受傳入連線
    // 使用 blocking iterator
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                // 為每個連線啟動新執行緒
                // 使用 move 將 stream 所有權移轉至執行緒
                let thread_backend = sync::Arc::clone(&backend);
                let _handle = std::thread::spawn(move || {
                    handle_request_base(stream, thread_backend);
                });
            }
            Err(e) => {
                eprintln!("連線失敗：{}", e);
            }
        }
    }
}

// 處理單一客戶端連線的函式
fn handle_request_base(
    stream: TcpStream,
    backend: sync::Arc<sync::RwLock<gamemodes::mode_base::PositiveMahjong>>,
) {
    // 進行 WebSocket 握手，建立 WebSocket 物件
    // 顯式宣告類型以符合規範
    let mut websocket: WebSocket<TcpStream> = match accept(stream) {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("握手失敗：{}", e);
            return;
        }
    };

    println!("客戶端連線成功");

    // 進入訊息接收迴圈
    loop {
        // 讀取訊息
        let msg: Result<Message, Error> = websocket.read();

        match msg {
            Ok(message) => {
                match message {
                    Message::Text(text) => {
                        println!("收到文字訊息：{}", text);
                        // 回覆訊息
                        let reply: Message = Message::Text(format!("伺服器收到：{}", text).into());
                        let _write_result: Result<(), Error> = websocket.write(reply);
                    }
                    Message::Binary(data) => {
                        println!("收到二進位資料，長度：{}", data.len());
                    }
                    Message::Ping(_) => {
                        // 函式庫通常會自動處理 Pong，亦可手動處理
                    }
                    Message::Pong(_) => {
                        // 忽略 Pong
                    }
                    Message::Close(_) => {
                        println!("客戶端請求關閉連線");
                        break;
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
    }

    // 關閉連線
    let _close_result: Result<(), Error> = websocket.close(None);
    println!("連線已終止");
}

fn main_base() {
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
    servers.push(handle_server_base(
        server_addr_ipv4,
        sync::Arc::clone(&backend),
    ));
    servers.push(handle_server_base(
        server_addr_ipv6,
        sync::Arc::clone(&backend),
    ));
}
