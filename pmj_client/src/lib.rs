use slint;
use slint::ComponentHandle;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod client;

// 引入 Slint 模組
//slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn ui_main() {
    let ui = client::main();
    ui.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = client::main();
    client::MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
