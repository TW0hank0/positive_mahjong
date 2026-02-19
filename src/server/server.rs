use local_ip_address;
use serde_json;
use std;
use std::sync;
use tiny_http;

use positive_mahjong::shared;

fn main() {
    let server = tiny_http::Server::http(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::UNSPECIFIED,
        10066,
    ))
    .unwrap();
    let backend = sync::Arc::new(sync::Mutex::new(PositiveMahjong::new()));
    println!("ip: {}", local_ip_address::local_ip().unwrap());
    for request in server.incoming_requests() {
        // TODO: std::thread
        handle_requset(request, sync::Arc::clone(&backend));
    }
}

fn handle_requset(
    mut request: tiny_http::Request,
    backend: sync::Arc<sync::Mutex<PositiveMahjong>>,
) {
    let mut content_string = String::new();
    request
        .as_reader()
        .read_to_string(&mut content_string)
        .unwrap();
    let content_data_result: Result<shared::ClientRequestType, serde_json::Error> =
        serde_json::from_str(&content_string);
    let mut guard = backend.lock().unwrap();
    match content_data_result {
        Ok(content_data) => {
            println!("{:?}", content_data);
            if content_data.app == "positive_mahjong" {
                match content_data.data.req_type {
                    shared::ActionType::AddPlayer => {
                        //let mut guard = backend.lock().unwrap();
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
                        //let mut guard = backend.lock().unwrap();
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct PMJPlayer {
    ip: std::net::SocketAddr,
    number: u8,
}

impl std::fmt::Display for PMJPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PMJPlayer:
    ip: {}
    number: {}
",
            self.ip.to_string(),
            self.number.to_string()
        )
    }
}

impl PositiveMahjong {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            player_count: 0,
            max_player_count: 4,
        }
    }

    pub fn add_player(&mut self, ip: std::net::SocketAddr) -> Either<String, u8> {
        if self.player_count >= self.max_player_count {
            return Either::Left(String::from("人數已達上限！"));
        } else {
            self.player_count += 1;
            self.players.push(PMJPlayer {
                ip: ip,
                number: self.player_count,
            });
            return Either::Right(self.player_count.clone());
        }
    }

    pub fn remove_player(&mut self, ip: std::net::SocketAddr, number: u8) -> Either<String, ()> {
        if self.players.contains(&PMJPlayer {
            ip: ip,
            number: number,
        }) {
            self.players.retain(|&x| {
                x != PMJPlayer {
                    ip: ip,
                    number: number,
                }
            });
            self.player_count -= 1;
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }
}
