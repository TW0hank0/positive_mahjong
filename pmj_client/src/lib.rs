mod client;

#[unsafe(no_mangle)]
pub fn android_main() {
    let _ = client::main();
}
