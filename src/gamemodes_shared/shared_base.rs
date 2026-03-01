use std::net;
use std::sync;

use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PMJPlayer {
    ip_addr: std::net::IpAddr,
}
