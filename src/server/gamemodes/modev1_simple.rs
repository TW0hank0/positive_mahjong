//! 玩法v1 - `simple`

use rand;
use rand::{prelude::SliceRandom, seq::IndexedRandom};

use positive_mahjong::shared::{self, GameActionAfter, GameActionPlayerRound, GameActionWaitRound};
use positive_mahjong::shared::{PMJCard, PMJCardFlowers, PMJCardTypes, PMJCardWords};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct PositiveMahjong {
    players: Vec<PMJPlayer>,
    player_count: u8,
    max_player_count: u8,
    is_start: bool,
    unuse_cards: Vec<PMJCard>,
    game_status: GameStatus,
    msg_queue_wait: sync::Arc<sync::RwLock<Vec<PlayerWaitRoundAction>>>,
    msg_queue_player: sync::Arc<sync::RwLock<Vec<PlayerPlayerRoundAction>>>,
    msg_queue_after: sync::Arc<sync::RwLock<Vec<AfterAction>>>,
}

#[derive(Debug)]
pub struct PlayerWaitRoundAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionWaitRound,
}

impl std::cmp::PartialEq for PlayerWaitRoundAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for PlayerWaitRoundAction {}

#[derive(Debug)]
pub struct AfterAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionAfter,
}

impl std::cmp::PartialEq for AfterAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for AfterAction {}

#[derive(Debug)]
pub struct PlayerPlayerRoundAction {
    pub player_ip: std::net::SocketAddr,
    pub player_number: u8,
    pub action: GameActionPlayerRound,
}

impl std::cmp::PartialEq for PlayerPlayerRoundAction {
    fn eq(&self, other: &Self) -> bool {
        self.player_number == other.player_number && self.action == other.action
    }
}

impl std::cmp::Eq for PlayerPlayerRoundAction {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GameStatus {
    pub rounds_count: u16,
    pub round_type: GameRoundType,
    /// 當 `round_type` 是 `GameRoundType::WaitRound` 時，
    /// `current_player_number` 是最後一個動作的玩家的 `player_number`
    pub current_player_number: u8,
    /// 回合是否結束
    pub is_round_finish: bool,
    /// 遊戲是否結束
    pub is_game_finish: bool,
    /// 最後一個丟的牌
    pub last_throw_card: Option<PMJCard>,
    pub game_winner: Option<PMJPlayer>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum GameRoundType {
    /// 玩家時間
    ///
    /// 可以：抽牌
    ///
    /// 不可：碰、吃、槓、暗槓、明槓、補花
    PlayerRoound,
    /// 等待時間
    ///
    /// 可以：碰、吃、槓、暗槓、明槓、補花
    ///
    /// 不可：抽牌
    WaitRound,
}

impl PositiveMahjong {
    pub fn new() -> Self {
        let mut unuse_cards = Vec::new();
        for card_type in vec![
            PMJCardTypes::TenThousand,
            PMJCardTypes::Line,
            PMJCardTypes::Dots,
        ] {
            for number in 1..10 {
                for card_id in 1..5 {
                    unuse_cards.push(PMJCard {
                        card_type: card_type,
                        card_number: number,
                        card_id: card_id,
                    });
                }
            }
        }
        for word in PMJCardWords::get_all() {
            for card_id in 1..5 {
                unuse_cards.push(PMJCard {
                    card_type: PMJCardTypes::Words(word),
                    card_number: 0,
                    card_id: card_id,
                });
            }
        }
        for flower in PMJCardFlowers::get_all() {
            unuse_cards.push(PMJCard {
                card_type: PMJCardTypes::Flower(flower),
                card_number: 0,
                card_id: 1,
            });
        }
        //
        Self {
            players: Vec::new(),
            player_count: 0,
            max_player_count: 4,
            is_start: false,
            unuse_cards: unuse_cards,
            game_status: GameStatus {
                rounds_count: 0,
                round_type: GameRoundType::WaitRound,
                current_player_number: 0,
                is_round_finish: false,
                is_game_finish: false,
                last_throw_card: None,
            },
            msg_queue_player: sync::Arc::new(sync::RwLock::new(Vec::new())),
            msg_queue_wait: sync::Arc::new(sync::RwLock::new(Vec::new())),
            msg_queue_after: sync::Arc::new(sync::RwLock::new(Vec::new())),
        }
    }

    pub fn add_player(&mut self, ip: std::net::SocketAddr) -> Either<String, u8> {
        if self.player_count >= self.max_player_count {
            return Either::Left(String::from("人數已達上限！"));
        } else if self.is_start {
            return Either::Left(String::from("遊戲已開始！"));
        } else {
            self.player_count += 1;
            self.players.push(PMJPlayer {
                ip: ip,
                number: self.player_count,
                cards: Vec::new(),
            });
            return Either::Right(self.player_count.clone());
        }
    }

    pub fn remove_player(&mut self, ip: std::net::SocketAddr, number: u8) -> Either<String, ()> {
        if self.is_start {
            return Either::Left(String::from("遊戲已開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: ip,
            number: number,
            cards: Vec::new(),
        }) {
            self.players.retain(|x: &PMJPlayer| {
                x.clone()
                    != PMJPlayer {
                        ip: ip,
                        number: number,
                        cards: Vec::new(),
                    }
            });
            self.player_count -= 1;
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    pub fn is_start(&self) -> bool {
        return self.is_start.clone();
    }

    pub fn start_game(&mut self) {
        let mut rng = rand::rng();
        self.unuse_cards.shuffle(&mut rng);
        //
        for _ in 0..4 {
            for player_number in 0..self.players.len() {
                /* let player = self.players.get_mut(player_number).unwrap();
                for _ in 0..4 {
                    let card = self.unuse_cards.choose(&mut rng).unwrap().clone();
                    let mut index = 0;
                    for unuse_card in self.unuse_cards.iter() {
                        if unuse_card.clone() == card.clone() {
                            break;
                        } else {
                            index += 1;
                        }
                    }
                    self.unuse_cards.remove(index);
                    player.cards.push(card);
                } */
                self.get_one_unuse_card(player_number);
            }
        }
        //莊家多一張
        self.get_one_unuse_card(0);
        //
        //TODO:等待過補
        self.game_loop();
    }

    pub fn replacing_a_flower(
        &mut self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let player = self.players.get_mut(player_number as usize).unwrap();
            let mut flower_card_indexes: Vec<usize> = Vec::new();
            let card_index: usize = 0;
            for card in player.cards.iter() {
                match card.card_type {
                    PMJCardTypes::Flower(_) => {
                        flower_card_indexes.push(card_index.clone());
                    }
                    _ => {} //其他忽略
                }
            }
            if !flower_card_indexes.is_empty() {
                return Either::Left(String::from("無花可補！"));
            } else {
                for index in flower_card_indexes.iter() {
                    player.cards.remove(index.clone());
                }
                for _ in 0..flower_card_indexes.len() {
                    self.get_one_unuse_card(player_number as usize);
                }
                return Either::Right(());
            }
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    /// for client
    pub fn get_cards(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
    ) -> Either<String, Vec<PMJCard>> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let player = self.players.get(player_number as usize).unwrap();
            return Either::Right(player.cards.clone());
        } else {
            return Either::Left(String::from("無此玩家！"));
        }
    }

    /// for client
    pub fn get_game_status(&self) -> GameStatus {
        return self.game_status.clone();
    }

    pub fn game_loop(&mut self) {
        loop {
            for round_player_number in 0..self.players.len() {
                for _round_type in vec![GameRoundType::PlayerRoound, GameRoundType::WaitRound] {
                    if !self.game_status.is_game_finish {
                        match self.game_status.round_type {
                            GameRoundType::PlayerRoound => {
                                self.handle_game_round_player(round_player_number as u8);
                            }
                            GameRoundType::WaitRound => {
                                self.handle_game_round_wait(round_player_number as u8);
                            }
                        }
                        self.change_game_round_type();
                    } else {
                        break;
                    }
                }
                if self.game_status.is_game_finish {
                    break;
                }
            }
            if self.game_status.is_game_finish {
                println!("遊戲已結束！");
                break;
            } else {
                self.game_status.rounds_count += 1;
            }
        }
    }

    fn check_win(&mut self) {
        'player: for player in self.players.iter() {
            let mut finish_check_cards = Vec::new();
            for card in player.cards {
                if finish_check_cards.contains(&card) {
                    continue;
                } else {
                    if card.card_type == PMJCardTypes::Dots
                        || card.card_type == PMJCardTypes::Line
                        || card.card_type == PMJCardTypes::TenThousand
                    {
                        todo!();
                        return false;
                    }
                }
            }
        }
    }

    fn check_win_have_card(
        &self,
        player_number: u8,
        card_type: PMJCardTypes,
        card_number: u8,
    ) -> Option<u8> {
        let player = self.players.get(player_number as usize).unwrap();
        let mut index: u8 = 0;
        let mut found_card: bool = false;
        for card in player.cards.iter() {}
        if found_card {
            return Some(index);
        } else {
            return None;
        }
    }

    fn change_game_round_type(&mut self) {
        match self.game_status.round_type {
            GameRoundType::PlayerRoound => {
                self.game_status.round_type = GameRoundType::WaitRound;
            }
            GameRoundType::WaitRound => {
                self.game_status.round_type = GameRoundType::PlayerRoound;
            }
        }
    }

    /// for client
    pub fn player_round_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionPlayerRound,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_player);
            let mut guard = queue_arc.write().unwrap();
            guard.push(PlayerPlayerRoundAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn wait_round_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionWaitRound,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_wait);
            let mut guard = queue_arc.write().unwrap();
            guard.push(PlayerWaitRoundAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn player_round_after_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionAfter,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_after);
            let mut guard = queue_arc.write().unwrap();
            guard.push(AfterAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    /// for client
    pub fn wait_round_after_action(
        &self,
        player_ip: std::net::SocketAddr,
        player_number: u8,
        action: GameActionAfter,
    ) -> Either<String, ()> {
        if !self.is_start {
            return Either::Left(String::from("遊戲未開始！"));
        } else if self.players.contains(&PMJPlayer {
            ip: player_ip,
            number: player_number,
            cards: Vec::new(),
        }) {
            let queue_arc = sync::Arc::clone(&self.msg_queue_after);
            let mut guard = queue_arc.write().unwrap();
            guard.push(AfterAction {
                player_ip: player_ip,
                player_number: player_number,
                action: action,
            });
            return Either::Right(());
        } else {
            return Either::Left(String::from("無此玩家或非此玩家回合！"));
        }
    }

    fn handle_game_round_player(&mut self, player_number: u8) {
        let duration = std::time::Duration::from_secs(1);
        {
            let queue_arc_player = sync::Arc::clone(&self.msg_queue_player);
            let mut exit_loop: bool = false;
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_player.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionPlayerRound::DrawATile => {
                                    self.get_one_unuse_card(player_number as usize);
                                    exit_loop = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            {
                let mut guard = queue_arc_player.write().unwrap();
                guard.clear();
            }
        }
        //
        {
            let mut exit_loop: bool = false;
            let queue_arc_after = sync::Arc::clone(&self.msg_queue_after);
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_after.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionAfter::Throw(card) => {
                                    if self
                                        .players
                                        .get(player_number as usize)
                                        .unwrap()
                                        .cards
                                        .contains(&card)
                                    {
                                        let player =
                                            self.players.get_mut(player_number as usize).unwrap();
                                        let mut index: usize = 0;
                                        //FIXME:不太可能出現的邏輯問題
                                        for i in player.cards.iter() {
                                            if i == &card {
                                                break;
                                            } else {
                                                index += 1;
                                            }
                                        }
                                        player.cards.remove(index);
                                        self.game_status.last_throw_card = Some(card);
                                        exit_loop = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            let mut guard = queue_arc_after.write().unwrap();
            guard.clear();
        }
    }

    fn have_card(&self, player_number: u8, card: &PMJCard, do_not_care_id: bool) -> bool {
        if do_not_care_id {
            for i in self
                .players
                .get(player_number as usize)
                .unwrap()
                .cards
                .iter()
            {
                if i.card_number == card.card_number && i.card_type == card.card_type {
                    return true;
                }
            }
            return false;
        } else {
            self.players
                .get(player_number as usize)
                .unwrap()
                .cards
                .contains(card)
        }
    }

    fn handle_game_round_wait(&self, player_number: u8) {
        let duration = std::time::Duration::from_secs(1);
        let mut is_need_after_throw: bool = false;
        {
            let queue_arc_wait = sync::Arc::clone(&self.msg_queue_wait);
            let mut exit_loop: bool = false;
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_wait.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if !(action.player_number == player_number) {
                            is_need_after_throw = true;
                            match action.action {
                                //FIXME
                                GameActionWaitRound::Eat(card) => {
                                    if self.game_status.last_throw_card.is_some()
                                        && card == self.game_status.last_throw_card.unwrap()
                                        && card.card_number > 0
                                        && ((card.card_number > 1
                                            && card.card_number < 9
                                            && self.have_card(
                                                action.player_number,
                                                &PMJCard {
                                                    card_type: card.card_type,
                                                    card_number: card.card_number - 1,
                                                    card_id: 0,
                                                },
                                                true,
                                            )
                                            && self.have_card(
                                                action.player_number,
                                                &PMJCard {
                                                    card_type: card.card_type,
                                                    card_number: card.card_number + 1,
                                                    card_id: 0,
                                                },
                                                true,
                                            ))
                                            || (card.card_number == 1
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number + 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number + 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                ))
                                            || (card.card_number == 9
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number - 1,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )
                                                && self.have_card(
                                                    action.player_number,
                                                    &PMJCard {
                                                        card_type: card.card_type,
                                                        card_number: card.card_number - 2,
                                                        card_id: 0,
                                                    },
                                                    true,
                                                )))
                                    {
                                        exit_loop = true;
                                        self.game_status.last_throw_card = None;
                                        todo!("Not Finish") //TODO
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            {
                let mut guard = queue_arc_wait.write().unwrap();
                guard.clear();
            }
        }
        //
        if is_need_after_throw {
            let mut exit_loop: bool = false;
            let queue_arc_after = sync::Arc::clone(&self.msg_queue_after);
            loop {
                std::thread::sleep(duration);
                let guard = queue_arc_after.read().unwrap();
                if !guard.is_empty() {
                    for action in guard.iter() {
                        if action.player_number == player_number {
                            match action.action {
                                GameActionAfter::Throw(card) => {
                                    if self
                                        .players
                                        .get(player_number as usize)
                                        .unwrap()
                                        .cards
                                        .contains(&card)
                                    {
                                        let player =
                                            self.players.get_mut(player_number as usize).unwrap();
                                        let mut index: usize = 0;
                                        //FIXME:不太可能出現的邏輯問題
                                        for i in player.cards.iter() {
                                            if i == &card {
                                                break;
                                            } else {
                                                index += 1;
                                            }
                                        }
                                        player.cards.remove(index);
                                        exit_loop = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if exit_loop {
                    break;
                }
            }
            let mut guard = queue_arc_after.write().unwrap();
            guard.clear();
        }
    }

    fn get_one_unuse_card(&mut self, player_number: usize) {
        let mut rng = rand::rng();
        let player = self.players.get_mut(player_number).unwrap();
        for _ in 0..4 {
            let card = self.unuse_cards.choose(&mut rng).unwrap().clone();
            let mut index = 0;
            for unuse_card in self.unuse_cards.iter() {
                if unuse_card.clone() == card.clone() {
                    break;
                } else {
                    index += 1;
                }
            }
            self.unuse_cards.remove(index);
            player.cards.push(card);
        }
    }
}

impl std::fmt::Display for PositiveMahjong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PositiveMahjong
    players: {:#?}
    player_count: {}
    max_player_count: {}",
            self.players,
            self.player_count.clone(),
            self.max_player_count
        )
    }
}
