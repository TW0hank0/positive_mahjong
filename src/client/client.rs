use positive_mahjong::shared;
use reqwest;
use serde_json;
use std;

//#[tokio::main]
fn main() {
    let mut server_ip = String::new();
    println!("input ipv4:");
    std::io::stdin().read_line(&mut server_ip).ok();
    let server_url = format!("http://{}:10066/", server_ip.clone());
    //println!("get ip: {}", server_ip);
    //let server_ip = "localhost";
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
        .post(server_url)
        .body(request)
        .timeout(std::time::Duration::from_mins(1))
        .send()
        .unwrap();
    //.await?;
    let body = response.text().unwrap(); //.await?;
    println!("回應: {}", body);
    /* // add player
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
        .unwrap(); //.await?;
    let body = response.text().unwrap(); //.await?;
    println!("回應: {}", body);
    let body_data: shared::ServerResponseType = serde_json::from_str(&body).unwrap();
    let number_1 = body_data.data.data_add_player.unwrap().number;
    // remove player
    std::thread::sleep(std::time::Duration::from_secs(1));
    let request_data = shared::ClientRequestDataType {
        req_type: shared::ActionType::RemovePlayer,
        data_remove_player: Some(shared::ClientRequestDataRemovePlayerType { number: number_1 }),
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
        .unwrap(); //.await?;
    let body = response.text().unwrap(); //.await?;
    println!("回應: {}", body); */
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
            .unwrap(); //.await?;
        let body = response.text().unwrap(); //.await?;
        println!("回應: {}", body);
    }
    //let body_data: shared::ServerResponseType = serde_json::from_str(&body).unwrap();
    //Ok(())
}
