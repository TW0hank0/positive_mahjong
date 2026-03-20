use std;

use reqwest;

// TODO

pub fn post_url(url: impl Into<String>, body: String) -> String {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();
    client
        .post(url.into())
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap()
}
