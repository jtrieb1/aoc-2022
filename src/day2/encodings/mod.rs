use std::{str::FromStr, fmt::Display};

mod game;
mod natural;
mod loss_draw_win;

use game::Round;
pub use natural::NaturalEncodingStrategy;
pub use loss_draw_win::LossDrawWinEncodingStrategy;

pub trait InstructionParsingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round;
}

pub struct EncodedInstruction(EncodedOpponentMove, EncodedPlayerMove);

pub enum EncodedOpponentMove {
    A, B, C
}

pub enum EncodedPlayerMove {
    X, Y, Z
}

impl FromStr for EncodedInstruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instrs = s.split(' ').take(2).into_iter();
        let opponent = match instrs.next().ok_or(InstructionParseError("Invalid instruction line"))? {
            "A" => EncodedOpponentMove::A,
            "B" => EncodedOpponentMove::B,
            _ => EncodedOpponentMove::C
        };
        let player = match instrs.next().ok_or(InstructionParseError("Invalid instruction line"))? {
            "X" => EncodedPlayerMove::X,
            "Y" => EncodedPlayerMove::Y,
            _ => EncodedPlayerMove::Z
        };
        Ok(Self(opponent, player))
    }
}

#[derive(Debug)]
pub struct InstructionParseError(&'static str);

impl Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl std::error::Error for InstructionParseError {}