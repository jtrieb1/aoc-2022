use crate::{parse_lines_into, read_input_to_str, AOCSolution};

use super::encodings::*;

pub struct RPSStrategyGuide {
    instructions: Vec<EncodedInstruction>,
}

impl AOCSolution for RPSStrategyGuide {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        let encoding = NaturalEncodingStrategy {};
        let score = self.get_total_score_using_encoding(&encoding);
        format!("{}", score)
    }

    fn part_2(&mut self) -> String {
        let encoding = LossDrawWinEncodingStrategy {};
        let score = self.get_total_score_using_encoding(&encoding);
        format!("{}", score)
    }
}

impl RPSStrategyGuide {
    fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions = parse_lines_into::<EncodedInstruction>(input_str)?;
        Ok(Self { instructions })
    }

    fn get_total_score_using_encoding(&self, encoding: &dyn InstructionParsingStrategy) -> u32 {
        self.instructions
            .iter()
            .map(|inst| encoding.parse_encoded(inst).score())
            .sum()
    }
}
