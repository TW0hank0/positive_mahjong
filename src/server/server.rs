use local_ip_address;
use serde_json;
use std;
use tiny_http;

use positive_mahjong::shared;

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:3000").unwrap();
    let mut backend = PositiveMahjong::new();
    println!("ip: {}", local_ip_address::local_ipv6().unwrap());
    for mut request in server.incoming_requests() {
        let mut content_string = String::new();
        request
            .as_reader()
            .read_to_string(&mut content_string)
            .unwrap();
        let content_data_result: Result<shared::ClientRequestType, serde_json::Error> =
            serde_json::from_str(&content_string);
        match content_data_result {
            Ok(content_data) => {
                println!("{:?}", content_data);
            }
            Err(e) => {
                let response = tiny_http::Response::from_string(data)
            }
        }
        let response = tiny_http::Response::from_string("Hello, World!");
        request.respond(response).unwrap();
    }
}

struct PositiveMahjong {
    players: Vec<PMJPlayer>,
}

struct PMJPlayer {
    ip: std::net::SocketAddr,
    number: u8,
}

impl PositiveMahjong {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }
}
