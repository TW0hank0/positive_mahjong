use local_ip_address;
use serde_json;
use std;
use std::sync;
use tiny_http;

use rand;
use rand::{prelude::SliceRandom, seq::IndexedRandom};

use positive_mahjong::shared;
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
    //let server_addr_ipv4 = "http://localhost:10066";
    let server_addr_ipv6 = std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::LOCALHOST,
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
                handle_requset(request, arc_backend);
            });
        }
    })
}

fn handle_requset(
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
            for player in self.players.iter_mut() {
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
    }
}
