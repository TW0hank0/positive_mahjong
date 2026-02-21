use iced;
use positive_mahjong::shared::{self, ClientRequestDataIsStartType};
use reqwest;
use serde_json;
use std;

fn main() {
    let mut server_ip = String::new();
    let timeout_duration = std::time::Duration::from_secs(15);
    println!("輸入網路地址 (ip)：");
    std::io::stdin().read_line(&mut server_ip).ok();
    let server_url = format!("http://{}:{}/", server_ip.clone(), shared::SERVER_PORT);
    let client = reqwest::blocking::Client::new();
    //
    let request_data = shared::ClientRequestDataType {
        req_type: shared::ActionType::TestConnection,
        ..Default::default()
    };
    let request = serde_json::to_string(&shared::ClientRequestType {
        app: String::from("positive_mahjong"),
        client: String::from("pmj-client"),
        data: request_data,
    })
    .unwrap();
    let response = client
        .post(server_url.clone())
        .body(request)
        .timeout(timeout_duration.clone())
        .send()
        .unwrap();
    //
    let body = response.text().unwrap();
    println!("回應: {}", body);
    //
    std::thread::sleep(std::time::Duration::from_secs(1));
    let request_data = shared::ClientRequestDataType {
        req_type: shared::ActionType::AddPlayer,
        ..Default::default()
    };
    let request = serde_json::to_string(&shared::ClientRequestType {
        app: String::from("positive_mahjong"),
        client: String::from("pmj-client"),
        data: request_data,
    })
    .unwrap();
    let response = client
        .post(server_url.clone())
        .body(request)
        .timeout(timeout_duration.clone())
        .send()
        .unwrap(); //.await?;
    let body = response.text().unwrap(); //.await?;
    println!("回應: {}", body);
    let body_data: shared::ServerResponseType = serde_json::from_str(&body).unwrap();
    let number = body_data.data.data_add_player.unwrap().number;
    //
    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let request_data = shared::ClientRequestDataType {
            req_type: shared::ActionType::IsStart,
            data_is_start: Some(ClientRequestDataIsStartType {
                number: number.clone(),
            }),
            ..Default::default()
        };
        let request = serde_json::to_string(&shared::ClientRequestType {
            app: String::from("positive_mahjong"),
            client: String::from("pmj-client"),
            data: request_data,
        })
        .unwrap();
        let response = client
            .post(server_url.clone())
            .body(request)
            .timeout(timeout_duration.clone())
            .send()
            .unwrap(); //.await?;
        let body = response.text().unwrap(); //.await?;
        println!("回應: {}", body);
    }
}

pub enum PMJClientMsg {}

struct PMJClient {}

impl PMJClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, message: PMJClientMsg) {}

    pub fn view(&self) {}
}
