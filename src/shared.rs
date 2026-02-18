use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientRequestType {
    app: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerResponseType {
    app: String,
}
