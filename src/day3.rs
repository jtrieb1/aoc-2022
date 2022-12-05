use crate::util::{read_input_to_str, parse_lines_into, AOCSolution};
use std::{collections::HashSet, str::{FromStr, Chars}, fmt::Display};

solution!(Day 3 => RucksackCollection);

pub struct RucksackCollection {
    rucksacks: Vec<Rucksack>
}

impl AOCSolution for RucksackCollection {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
        where
            Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        format!("{}", self.score())
    }

    fn part_2(&mut self) -> String {
        format!("{}", self.badge_score())
    }
}

impl RucksackCollection {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let rucksacks = parse_lines_into::<Rucksack>(input_str)?;
        Ok(Self {
            rucksacks
        })
    }

    pub fn score(&self) -> u32 {
        self.rucksacks.iter().map(|r| r.score()).sum()
    }

    pub fn badge_score(&self) -> u32 {
        self.rucksacks.chunks(3)
            .map(|group| group[0].find_badge_char(&group[1], &group[2]).unwrap())
            .map(|badge| char_score(&badge))
            .sum()
    }
}

struct Rucksack {
    left: Compartment,
    right: Compartment
}

impl FromStr for Rucksack {
    type Err = RucksackParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(RucksackParseError("Invalid string length for rucksack entry."));
        }
        let idx = s.len() / 2;
        let (left_str, right_str) = (&s[..idx], &s[idx..]);
        let left = left_str.parse::<Compartment>()?;
        let right = right_str.parse::<Compartment>()?;
        Ok(Self {
            left,
            right
        })
    }
}

impl Rucksack {
    pub fn score(&self) -> u32 {
        for c in self.right.chars() {
            if self.left.check_present(c) {
                return char_score(&c);
            }
        }

        return 0;
    }

    pub fn find_badge_char(&self, second: &Rucksack, third: &Rucksack) -> Option<char> {
        let mut candidates = HashSet::new();
        for c in second.chars() {
            if self.has_char(&c) {
                candidates.insert(c);
            }
        }
        for c in third.chars() {
            if candidates.contains(&c) {
                return Some(c);
            }
        }
        None
    }

    fn chars(&self) -> Vec<char> {
        self.left.chars().chain(self.right.chars()).collect()
    }

    fn has_char(&self, c: &char) -> bool {
        self.left.check_present(*c) || self.right.check_present(*c)
    }
}

struct Compartment {
    repr: String,
    manifest: HashSet<char>
}

impl FromStr for Compartment {
    type Err = RucksackParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let repr = s.to_string();
        let mut manifest = HashSet::new();
        for c in s.chars() {
            manifest.insert(c);
        }
        Ok(Self {
            repr,
            manifest
        })
    }
}

impl Compartment {
    pub fn check_present(&self, c: char) -> bool {
        self.manifest.contains(&c)
    }

    pub fn chars(&self) -> Chars<'_> {
        self.repr.chars()
    }
}

#[derive(Debug)]
struct RucksackParseError(&'static str);

impl Display for RucksackParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl std::error::Error for RucksackParseError {}

fn char_score(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 65 + 26 + 1
    } else {
        *c as u32 - 97 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let input = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        let coll = RucksackCollection::new_from_str(input).expect("Failed to parse rucksack");
        let score = coll.score();
        assert_eq!(score, 157);
        let badge_score = coll.badge_score();
        assert_eq!(badge_score, 70);
    }
}