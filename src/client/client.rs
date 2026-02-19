use positive_mahjong::shared;
use reqwest;
use serde_json;
use std;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server_ip = String::new();
    println!("input ipv4:");
    std::io::stdin().read_line(&mut server_ip).ok();
    println!("get ip: {}", server_ip);
    //let server_ip = "localhost";
    let client = reqwest::Client::new();
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
        .post(format!("http://{}:10066/", server_ip.clone()))
        .body(request)
        .timeout(std::time::Duration::from_mins(1))
        .send()
        .await?;
    let body = response.text().await?;
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
        .post(format!("http://{}:10066/", server_ip.clone()))
        .body(request)
        .timeout(std::time::Duration::from_mins(1))
        .send()
        .await?;
    let body = response.text().await?;
    println!("回應: {}", body);
    let body_data: shared::ServerResponseType = serde_json::from_str(&body).unwrap();
    let number = body_data.data.data_add_player.unwrap().number;
    //
    std::thread::sleep(std::time::Duration::from_secs(1));
    let request_data = shared::ClientRequestDataType {
        req_type: shared::ActionType::RemovePlayer,
        data_remove_player: Some(shared::ClientRequestDataRemovePlayerType { number: number }),
        ..Default::default()
    };
    let request = serde_json::to_string(&shared::ClientRequestType {
        app: String::from("positive_mahjong"),
        client: String::from("pmj-client"),
        data: request_data,
    })
    .unwrap();
    let response = client
        .post(format!("http://{}:10066/", server_ip.clone()))
        .body(request)
        .timeout(std::time::Duration::from_mins(1))
        .send()
        .await?;
    let body = response.text().await?;
    println!("回應: {}", body);
    Ok(())
}
