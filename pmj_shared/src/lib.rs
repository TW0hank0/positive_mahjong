//! pmj_shared

pub mod gamemodes_shared;
pub mod shared;

#[cfg(target_arch = "wasm32")]
pub mod async_net;

#[cfg(not(target_arch = "wasm32"))]
pub mod sync_net;
