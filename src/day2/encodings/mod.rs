use std::{fmt::Display, str::FromStr};

mod game;
mod loss_draw_win;
mod natural;

use game::Round;
pub use loss_draw_win::LossDrawWinEncodingStrategy;
pub use natural::NaturalEncodingStrategy;

pub trait InstructionParsingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round;
}

pub struct EncodedInstruction(EncodedOpponentMove, EncodedPlayerMove);

pub enum EncodedOpponentMove {
    A,
    B,
    C,
}

pub enum EncodedPlayerMove {
    X,
    Y,
    Z,
}

impl FromStr for EncodedInstruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instrs: [&str; 2] = s
            .split(' ')
            .take(2)
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| InstructionParseError("Invalid instruction string."))?;

        let opponent = instrs[0].parse::<EncodedOpponentMove>()?;
        let player = instrs[1].parse::<EncodedPlayerMove>()?;

        Ok(Self(opponent, player))
    }
}

impl FromStr for EncodedOpponentMove {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(EncodedOpponentMove::A),
            "B" => Ok(EncodedOpponentMove::B),
            "C" => Ok(EncodedOpponentMove::C),
            _ => Err(InstructionParseError(
                "Invalid opponent move. Expected 'A', 'B', or 'C'.",
            )),
        }
    }
}

impl FromStr for EncodedPlayerMove {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(EncodedPlayerMove::X),
            "Y" => Ok(EncodedPlayerMove::Y),
            "Z" => Ok(EncodedPlayerMove::Z),
            _ => Err(InstructionParseError(
                "Invalid player move. Expected 'X', 'Y', or 'Z'.",
            )),
        }
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
