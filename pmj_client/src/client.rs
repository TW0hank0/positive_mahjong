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

use slint::{self, ComponentHandle, SharedString, Weak};

use reqwest;
use std::{sync, thread};

use log;
use tungstenite::{Message, connect};

use pmj_shared::{
    gamemodes_shared::{self, shared_base},
    shared,
};

// 引入 Slint 模組
slint::include_modules!();

pub fn main() -> MainWindow {
    // 初始化視窗
    let main_window = MainWindow::new().unwrap();
    // 建立弱參考，用於子執行緒安全更新 UI
    let weak_window = main_window.as_weak();
    //
    //let timeout_duration = std::time::Duration::from_secs(10);
    // 設定Callback
    //let window_for_callback = main_window.clone_strong();
    main_window.window().set_maximized(true);
    let _ = main_window.window().show();
    //
    let thread_weak: Weak<MainWindow> = weak_window.clone();
    let _ = thread_weak.upgrade_in_event_loop(|upgraded_window| {
        upgraded_window.set_home_page_project_version(SharedString::from(format!(
            "v{}",
            shared::PROJECT_VERSION
        )));
        let window = upgraded_window.window();
        window.set_size(slint::WindowSize::Physical(slint::PhysicalSize {
            width: 1080,
            height: 720,
        }));
        window.set_position(slint::WindowPosition::Physical(slint::PhysicalPosition {
            x: 10,
            y: 10,
        }));
        if !window.is_maximized() {
            window.set_maximized(false);
        }
    });
    //
    main_window.on_home_page_add_player(move || {
        // 克隆弱參考給新執行緒
        let thread_weak: Weak<MainWindow> = weak_window.clone();
        let read_thread_weak = weak_window.clone();
        // 線程
        thread::spawn(move || {
            // 安全地回到主執行緒更新 UI
            thread_weak
                .upgrade_in_event_loop(move |upgraded_window| {
                    let input_server_ip: String = upgraded_window.get_home_page_server_ip().into();
                    //
                    if input_server_ip.is_empty() {
                        upgraded_window.set_home_page_server_response_text(SharedString::from(
                            format!(
                                "{} \n{}",
                                upgraded_window.get_home_page_server_response_text(),
                                "輸入錯誤：未輸入伺服器Ip！"
                            ),
                        ));
                    } else {
                        let server_url =
                            format!("ws://{}:{}/", input_server_ip, shared::SERVER_PORT);
                        let clone_server_url = server_url.clone();
                        println!("connecting to: {}", server_url.clone());
                        upgraded_window.set_home_page_server_response_text(SharedString::from(
                            format!(
                                "{} \n正在連線到伺服器 ({})...",
                                upgraded_window.get_home_page_server_response_text(),
                                clone_server_url
                            ),
                        ));
                        match connect(server_url) {
                            Ok((mut row_ws, _resp)) => {
                                let mut ws = sync::Arc::new(sync::RwLock::new(row_ws));
                                let req_text =
                                    serde_json::to_string(&shared::ClientConnectRequestType {
                                        app_name: String::from("positive_mahjong"),
                                        client: String::from("pmj_client"),
                                    })
                                    .unwrap();
                                let _ = ws.write().unwrap().send(Message::Text(req_text.into()));
                                let raw_msg = ws.write().unwrap().read().unwrap();
                                match raw_msg {
                                    Message::Text(text) => {
                                        let msg: shared::ServerConnectResponceType =
                                            serde_json::from_str(&text.to_string()).unwrap();
                                        if msg.too_many_player {
                                            upgraded_window.set_home_page_server_response_text(
                                                SharedString::from(format!(
                                                    "{}\n{}",
                                                    upgraded_window
                                                        .get_home_page_server_response_text(),
                                                    "加入失敗：人數過多！"
                                                )),
                                            );
                                        } else {
                                            let player_id = msg.player_id.unwrap();
                                            let gamemode = msg.gamemode;
                                            match gamemode {
                                                shared::GameModes::Base => {
                                                    upgraded_window
                                                        .set_home_page_server_response_text(
                                                            SharedString::from(format!(
                                                            "{}\n{}{}",
                                                            upgraded_window
                                                                .get_home_page_server_response_text(
                                                                ),
                                                            "已加入遊戲，玩家Id為",
                                                            player_id
                                                        )),
                                                        );
                                                    upgraded_window
                                                        .set_current_scene(Scenes::GamePlayBase);
                                                    upgraded_window.set_game_play_base_player_id(
                                                        player_id.into(),
                                                    );
                                                    let thread_ws = sync::Arc::clone(&ws);
                                                    let _thread_handle = thread::spawn(move || {
                                                        let row_msg = thread_ws.write().unwrap().read().unwrap();
                                                        match row_msg {
                                                            Message::Text(text) => {
                                                                let msg: shared_base::ServerMessageType = serde_json::from_str(&text).unwrap();
                                                                match msg.msg_type {
                                                                    shared_base::ServerMessageTypeKinds::GameStart => {}
                                                                    _ => {}
                                                                }
                                                            }
                                                            _ => {println!("只處理 Message::Text！");}
                                                        }
                                                        read_thread_weak;
                                                    });
                                                }
                                                shared::GameModes::V1Simple => {
                                                    upgraded_window
                                                        .set_home_page_server_response_text(
                                                            SharedString::from(format!(
                                                            "{} \n{}",
                                                            upgraded_window
                                                                .get_home_page_server_response_text(
                                                                ),
                                                            "不支援的玩法！"
                                                        )),
                                                        );
                                                }
                                                shared::GameModes::V2Better => {
                                                    upgraded_window
                                                        .set_home_page_server_response_text(
                                                            SharedString::from(format!(
                                                        "{} \n{}",
                                                        upgraded_window
                                                            .get_home_page_server_response_text(
                                                            ),
                                                        "不支援的玩法！"
                                                    )),
                                                        );
                                                }
                                            }
                                        }
                                    }
                                    _ => { /*忽略*/ }
                                }
                            }
                            Err(e) => {
                                upgraded_window.set_home_page_server_response_text(
                                    SharedString::from(format!(
                                        "{}\n{}{}",
                                        upgraded_window.get_home_page_server_response_text(),
                                        "伺服器連接失敗！err-msg:",
                                        e.to_string()
                                    )),
                                );
                            }
                        }
                    }
                })
                .ok();
        });
    });

    return main_window;
}
