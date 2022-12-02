use std::{str::FromStr, fmt::Display};

use crate::util::*;

solution!(Day 2 => RPSStrategyGuide);

pub struct RPSStrategyGuide {
    instructions: Vec<EncodedInstruction>,
}

impl AOCSolution for RPSStrategyGuide {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>> where Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        let encoding = NaturalEncodingStrategy {};
        let rounds = self.get_rounds_using_strategy(&encoding);
        let score = self.get_total_score_from_rounds(rounds);
        format!("{}", score)
    }

    fn part_2(&mut self) -> String {
        let encoding = LossDrawWinEncodingStrategy {};
        let rounds = self.get_rounds_using_strategy(&encoding);
        let score = self.get_total_score_from_rounds(rounds);
        format!("{}", score)
    }
}

impl RPSStrategyGuide {
    fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path)?;
        Self::new_from_str(&input_str)
    }

    fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions = parse_lines_into::<EncodedInstruction>(input_str)?;
        Ok(Self {
            instructions
        })
    }

    fn get_rounds_using_strategy(&self, strategy: &dyn InstructionParsingStrategy) -> Vec<Round> {
        self.instructions.iter().map(|inst| strategy.parse_encoded(inst)).collect()
    }

    fn get_total_score_from_rounds(&self, rounds: Vec<Round>) -> u32 {
        rounds.iter().map(|r| r.score()).sum()
    }

}

impl std::error::Error for InstructionParseError {}

struct EncodedInstruction(EncodedOpponentMove, EncodedPlayerMove);

enum EncodedOpponentMove {
    A, B, C
}

enum EncodedPlayerMove {
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
struct InstructionParseError(&'static str);

impl Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

struct Round {
    player: Hand,
    opponent: Hand
}

impl Round {
    const LOSS_SCORE: u32 = 0;
    const DRAW_SCORE: u32 = 3;
    const WIN_SCORE: u32 = 6;

    pub fn score(&self) -> u32 {
        if let Some(did_win) = self.player.does_beat(&self.opponent) {
            match did_win {
                true => self.player.score() + Self::WIN_SCORE,
                false => self.player.score() + Self::LOSS_SCORE
            }
        } else {
            self.player.score() + Self::DRAW_SCORE
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    pub fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    pub fn does_beat(&self, other: &Hand) -> Option<bool> {
        let win = self.beats();
        let loss = self.beaten_by();
        if *other == win {
            Some(true)
        } else if *other == loss {
            Some(false)
        } else {
            None
        }
    }

    fn beaten_by(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        }
    }

    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    pub fn get_hand_to_achieve(&self, outcome: &Outcome) -> Hand {
        match outcome {
            Outcome::Draw => self.clone(),
            Outcome::Win => self.beaten_by(),
            Outcome::Loss => self.beats()
        }
    }
}

enum Outcome {
    Loss,
    Win,
    Draw
}

trait InstructionParsingStrategy {
    fn parse_encoded(&self, inst: &EncodedInstruction) -> Round;
}

struct NaturalEncodingStrategy {}

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
        Round { player, opponent }
    }
}

struct LossDrawWinEncodingStrategy {}

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

        Round { player, opponent }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let input_str = "
        A Y
        B X
        C Z
        ";

        let mut guide = RPSStrategyGuide::new_from_str(input_str).expect("Failed to parse guide.");

        assert_eq!(guide.part_1(), "15");
        assert_eq!(guide.part_2(), "12");
    }
}