#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Hand {
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

    pub fn versus(&self, other: &Hand) -> Outcome {
        let win = self.beats();
        let loss = self.beaten_by();
        if *other == win {
            Outcome::Win
        } else if *other == loss {
            Outcome::Loss
        } else {
            Outcome::Draw
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

pub enum Outcome {
    Loss,
    Win,
    Draw
}