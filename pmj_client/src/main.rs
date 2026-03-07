use slint;

mod client;

// 引入 Slint 模組
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    client::main()
}
