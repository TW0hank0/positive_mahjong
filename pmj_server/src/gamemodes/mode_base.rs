use pmj_shared::gamemodes_shared;
use pmj_shared::shared;

pub struct PositiveMahjong {
    players: Vec<gamemodes_shared::shared_base::PMJPlayer>,
}

impl PositiveMahjong {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }
}
