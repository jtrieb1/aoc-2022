use std::str::FromStr;

pub struct EncodedInstruction(EncodedOpponentMove, EncodedPlayerMove);

impl EncodedInstruction {
    pub fn opponent(&self) -> EncodedOpponentMove {
        self.0
    }

    pub fn player(&self) -> EncodedPlayerMove {
        self.1
    }
}

#[derive(Clone, Copy)]
pub enum EncodedOpponentMove {
    A,
    B,
    C,
}

#[derive(Clone, Copy)]
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

custom_error!(InstructionParseError);
