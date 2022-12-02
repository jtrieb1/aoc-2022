use crate::util::*;

mod encodings;
mod strategy_guide;

use strategy_guide::RPSStrategyGuide;

solution!(Day 2 => RPSStrategyGuide);

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
