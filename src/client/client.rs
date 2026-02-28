use slint::{ComponentHandle, SharedString, Weak};
use std::thread;
use std::time::Duration;

// 引入 Slint 模組
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 1. 初始化視窗
    let main_window: MainWindow = MainWindow::new()?;

    // 2. 建立弱參考，用於子執行緒安全更新 UI
    let weak_window: Weak<MainWindow> = main_window.as_weak();

    // 3. 設定回調函式
    let window_for_callback = main_window.clone_strong();
    main_window.on_start_background_task(move || {
        let input_data: String = window_for_callback.get_user_input().into();

        // 更新 UI 為處理中
        window_for_callback.set_status_text(SharedString::from("處理中..."));
        window_for_callback.set_has_error(false);

        // 克隆弱參考給新執行緒
        let thread_weak: Weak<MainWindow> = weak_window.clone();

        // 4. 啟動標準執行緒 (禁止異步)
        thread::spawn(move || {
            // 模擬耗時的網路 POST 請求
            thread::sleep(Duration::from_secs(2));

            let result_message: String = if input_data.is_empty() {
                "錯誤：輸入不能為空".to_string()
            } else {
                format!("成功！已發送資料：{}", input_data)
            };

            let is_err: bool = input_data.is_empty();

            // 5. 安全地回到主執行緒更新 UI
            if let Some(upgraded_window) = thread_weak.upgrade() {
                upgraded_window
                    .invoke_from_event_loop(move |w| {
                        w.set_status_text(SharedString::from(result_message));
                        w.set_has_error(is_err);
                    })
                    .ok();
            }
        });
    });

    // 6. 運行事件迴圈
    main_window.run()?;
    Ok(())
}
