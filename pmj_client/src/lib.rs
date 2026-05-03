pub mod client;

#[cfg(feature = "android")]
mod android;
#[cfg(feature = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(feature = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android::entry::android_entry(android_app);
}
