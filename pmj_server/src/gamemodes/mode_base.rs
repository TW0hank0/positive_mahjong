use positive_mahjong::gamemodes_shared;
use positive_mahjong::shared;

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
