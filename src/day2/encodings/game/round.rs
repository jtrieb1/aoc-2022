use super::{Hand, Outcome};

pub struct Round {
    player: Hand,
    opponent: Hand
}

impl Round {
    const LOSS_SCORE: u32 = 0;
    const DRAW_SCORE: u32 = 3;
    const WIN_SCORE: u32 = 6;

    pub fn new(player: Hand, opponent: Hand) -> Self {
        Self { player, opponent }
    }

    pub fn score(&self) -> u32 {
        match self.player.versus(&self.opponent) {
            Outcome::Win => self.player.score() + Self::WIN_SCORE,
            Outcome::Draw => self.player.score() + Self::DRAW_SCORE,
            Outcome::Loss => self.player.score() + Self::LOSS_SCORE
        }
    }
}