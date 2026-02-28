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

//use rand;
//use rand::{prelude::SliceRandom, seq::IndexedRandom};

use positive_mahjong::gamemodes_shared;
use positive_mahjong::shared;
//use positive_mahjong::shared::{self, GameActionAfter, GameActionPlayerRound, GameActionWaitRound};
//use positive_mahjong::shared::{PMJCard, PMJCardFlowers, PMJCardTypes, PMJCardWords};

mod gamemodes;

const CURRENT_GAMEMODE: shared::GameModes = shared::GameModes::V1Simple;

fn main() {
    println!("ipv4: {}", local_ip_address::local_ip().unwrap());
    println!("ipv6: {}", local_ip_address::local_ipv6().unwrap());
    //
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
