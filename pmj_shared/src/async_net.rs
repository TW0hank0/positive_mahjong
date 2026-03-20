use reqwest;

// TODO

pub async fn post_url(url: impl Into<String>, body: String) -> String {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();
    client
        .post(url.into())
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
