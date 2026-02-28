use slint;
use slint::{ComponentHandle, SharedString, Weak};

use std::thread;
use std::time::Duration;

use reqwest;

use positive_mahjong::{gamemodes_shared, shared};

// 引入 Slint 模組
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 初始化視窗
    let main_window = MainWindow::new()?;
    // 建立弱參考，用於子執行緒安全更新 UI
    let weak_window: Weak<MainWindow> = main_window.as_weak();
    //
    let timeout_duration = std::time::Duration::from_secs(15);
    let client = reqwest::blocking::Client::new();
    // 設定Callback
    let window_for_callback = main_window.clone_strong();
    main_window.on_test_connection(move || {
        // 克隆弱參考給新執行緒
        let thread_weak: Weak<MainWindow> = weak_window.clone();
        //
        let input_server_ip: String = window_for_callback.get_server_ip().into();
        // 線程
        thread::spawn(move || {
            //
            let server_url = format!(
                "http://{}:{}/",
                input_server_ip.clone(),
                shared::SERVER_PORT
            );
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
            let resp_body_text = response.text().unwrap();
            // 安全地回到主執行緒更新 UI
            thread_weak
                .upgrade_in_event_loop(move |upgraded_window| {
                    upgraded_window.set_server_response_text(SharedString::from(resp_body_text));
                })
                .ok();
        });
    });

    // 事件迴圈
    main_window.run()?;
    Ok(())
}
