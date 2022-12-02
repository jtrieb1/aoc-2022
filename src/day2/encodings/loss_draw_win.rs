use super::*;
use game::{Hand, Outcome};

pub struct LossDrawWinEncodingStrategy {}

impl InstructionParsingStrategy for LossDrawWinEncodingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round {
        let opponent = match inst.0 {
            EncodedOpponentMove::A => Hand::Rock,
            EncodedOpponentMove::B => Hand::Paper,
            EncodedOpponentMove::C => Hand::Scissors
        };

        let player_outcome = match inst.1 {
            EncodedPlayerMove::X => Outcome::Loss,
            EncodedPlayerMove::Y => Outcome::Draw,
            EncodedPlayerMove::Z => Outcome::Win
        };

        let player = opponent.get_hand_to_achieve(&player_outcome);

        Round::new(player, opponent)
    }
}