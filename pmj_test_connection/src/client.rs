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
        req_action_type: shared::ActionType::TestConnection,
        ..Default::default()
    };
    let request = serde_json::to_string(&shared::ClientRequestType {
        app: String::from("positive_mahjong"),
        client: String::from("pmj-client"),
        data: request_data,
        game_data_v1: None,
        is_test_connection: true,
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
        req_action_type: shared::ActionType::AddPlayer,
        ..Default::default()
    };
    let request = serde_json::to_string(&shared::ClientRequestType {
        app: String::from("positive_mahjong"),
        client: String::from("pmj-client"),
        data: request_data,
        game_data_v1: None,
        is_test_connection: false,
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
            req_action_type: shared::ActionType::IsStart,
            data_is_start: Some(ClientRequestDataIsStartType {
                number: number.clone(),
            }),
            ..Default::default()
        };
        let request = serde_json::to_string(&shared::ClientRequestType {
            app: String::from("positive_mahjong"),
            client: String::from("pmj-client"),
            data: request_data,
            game_data_v1: None,
            is_test_connection: false,
        })
        .unwrap();
        let response = client
            .post(server_url.clone())
            .body(request)
            .timeout(timeout_duration.clone())
            .send()
            .unwrap();
        let body = response.text().unwrap();
        println!("回應: {}", body);
        let resp_data: shared::ServerResponseType = serde_json::from_str(&body).unwrap();
        if resp_data.data.data_is_start.is_some() && resp_data.data.data_is_start.unwrap().is_start
        {
            println!("這是測試連線的客戶端，無法遊玩！");
            break;
        };
    }
}
