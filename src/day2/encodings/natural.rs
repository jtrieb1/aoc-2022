use super::*;
use game::Hand;

pub struct NaturalEncodingStrategy {}

impl InstructionParsingStrategy for NaturalEncodingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round {
        let opponent = match inst.0 {
            EncodedOpponentMove::A => Hand::Rock,
            EncodedOpponentMove::B => Hand::Paper,
            EncodedOpponentMove::C => Hand::Scissors
        };
        let player = match inst.1 {
            EncodedPlayerMove::X => Hand::Rock,
            EncodedPlayerMove::Y => Hand::Paper,
            EncodedPlayerMove::Z => Hand::Scissors
        };
        Round::new(player, opponent)
    }
}