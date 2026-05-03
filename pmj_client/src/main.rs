#[cfg(feature = "desktop")]
use iced;

#[cfg(feature = "desktop")]
mod client;

fn main() {
    #[cfg(feature = "desktop")]
    {
        let _ = iced::application(
            client::Client::new,
            client::Client::update,
            client::Client::view,
        )
        .title(client::Client::title)
        .run();
    }
    #[cfg(not(feature = "desktop"))]
    {
        panic!("Must enable `desktop` feature on desktop platform (main.rs)!");
    }
}
