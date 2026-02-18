use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_ip = "localhost";
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}:3000/", server_ip))
        .body("{\"app\": \"PMJ-Client\"}")
        .timeout(std::time::Duration::from_mins(1))
        .send()
        .await?;
    let body = response.text().await?;
    println!("回應: {}", body);
    Ok(())
}
