use slint::{self, ComponentHandle, SharedString, Weak};

///use std::time::Duration;
use reqwest;
use std::thread;

use pmj_shared::{gamemodes_shared, shared};

#[cfg(not(target_arch = "wasm32"))]
use pmj_shared::sync_net as net;

#[cfg(target_arch = "wasm32")]
use pmj_shared::async_net as net;

// 引入 Slint 模組
slint::include_modules!();

pub fn main() -> MainWindow {
    // 初始化視窗
    let main_window = MainWindow::new().unwrap();
    // 建立弱參考，用於子執行緒安全更新 UI
    let weak_window = main_window.as_weak();
    //
    let timeout_duration = std::time::Duration::from_secs(10);
    // 設定Callback
    let window_for_callback = main_window.clone_strong();
    main_window.window().set_maximized(true);
    let _ = main_window.window().show();
    main_window.on_home_page_test_connection(move || {
        // 克隆弱參考給新執行緒
        let thread_weak: Weak<MainWindow> = weak_window.clone();
        // 線程
        thread::spawn(move || {
            // 安全地回到主執行緒更新 UI
            thread_weak
                .upgrade_in_event_loop(move |upgraded_window| {
                    let mut resp_body_text = String::new();
                    {
                        let input_server_ip: String =
                            upgraded_window.get_home_page_server_ip().into();
                        //let mut resp_body_text = String::new();
                        //
                        if input_server_ip.is_empty() {
                            resp_body_text.push_str("錯誤！未輸入正確伺服器Ip！");
                        } else {
                            let server_url =
                                format!("http://{}:{}/", input_server_ip, shared::SERVER_PORT);
                            let clone_server_url = server_url.clone();
                            println!("post to: {}", server_url.clone());
                            upgraded_window.set_home_page_server_response_text(SharedString::from(
                                format!("正在發送Post到伺服器 ({})...", clone_server_url),
                            ));
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
                            match client
                                .post(server_url.clone())
                                .body(request)
                                .timeout(timeout_duration.clone())
                                .send()
                            {
                                Ok(resp) => {
                                    let resp_text = resp.text().unwrap();
                                    let binding: Result<
                                        shared::ServerResponseType,
                                        serde_json::Error,
                                    > = serde_json::from_str(&resp_text);
                                    match binding {
                                        Ok(value) => {
                                            resp_body_text.push_str(
                                                &serde_json::to_string_pretty(&value)
                                                    .unwrap_or(String::from(resp_text)),
                                            );
                                        }
                                        Err(_e) => {
                                            /* log error */
                                            resp_body_text.push_str(&resp_text);
                                        }
                                    }
                                }
                                Err(e) => {
                                    resp_body_text.push_str(&format!("錯誤：{}", e.to_string()));
                                }
                            }
                            //
                        }
                    }
                    upgraded_window
                        .set_home_page_server_response_text(SharedString::from(resp_body_text));
                })
                .ok();
        });
    });

    return main_window;
}
