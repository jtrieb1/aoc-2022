use crate::util::{read_input_to_str, parse_lines_into, AOCSolution};
use std::{str::FromStr, fmt::Display};

solution!(Day 4 => CampSectionRegistry);

pub struct CampSectionRegistry {
    sections: Vec<CampSectionRecord>
}

impl AOCSolution for CampSectionRegistry {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
        where
            Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        format!("{}", self.num_redundant_sections())
    }

    fn part_2(&mut self) -> String {
        format!("{}", self.num_overlapping_sections())
    }
}

impl CampSectionRegistry {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let records = parse_lines_into::<CampSectionRecord>(input_str)?;
        Ok(Self {
            sections: records
        })
    }

    fn num_redundant_sections(&self) -> u32 {
        self.sections.iter()
            .map(|s| s.redundant_sections())
            .filter(|b| *b)
            .count() as u32
    }

    fn num_overlapping_sections(&self) -> u32 {
        self.sections.iter()
            .map(|s| s.overlapping_sections())
            .filter(|b| *b)
            .count() as u32
    }
}

struct CampSectionRecord {
    left: CampRange,
    right: CampRange
}

impl FromStr for CampSectionRecord {
    type Err = RangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: [&str; 2] = s.split(',').collect::<Vec<&str>>().try_into().map_err(|_| RangeParseError("Invalid record length"))?;
        let left = ranges[0].parse::<CampRange>()?;
        let right = ranges[1].parse::<CampRange>()?;
        Ok(Self {
            left,
            right
        })
    }
}

impl CampSectionRecord {
    pub fn redundant_sections(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    pub fn overlapping_sections(&self) -> bool {
        self.left.overlaps(&self.right) // This is invariant under permutation of arguments, so don't check right.overlaps(left)
    }
}

struct CampRange {
    min: u32,
    max: u32
}

impl FromStr for CampRange {
    type Err = RangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: [&str; 2] = s.split('-').collect::<Vec<&str>>().try_into().map_err(|_| RangeParseError("Invalid range string."))?;
        let min = sections[0].parse::<u32>().map_err(|_| RangeParseError("Invalid integer given."))?;
        let max = sections[1].parse::<u32>().map_err(|_| RangeParseError("Invalid integer given."))?;
        Ok(Self {
            min,
            max
        })
    }
}

impl CampRange {
    pub fn overlaps(&self, other: &CampRange) -> bool {
        // [ (] )
        (self.max >= other.min && other.min >= self.min) ||
        // ( [) ]
        (self.min <= other.max && other.max <= self.max) ||
        // [ () ]
        self.contains(other) ||
        // ( [] )
        other.contains(self)
    }

    pub fn contains(&self, other: &CampRange) -> bool {
        self.min <= other.min && self.max >= other.max
    }
}

#[derive(Debug)]
struct RangeParseError(&'static str);

impl Display for RangeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl std::error::Error for RangeParseError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let input = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        ";

        let reg = CampSectionRegistry::new_from_str(input).expect("Unable to parse registry");
        assert_eq!(reg.num_redundant_sections(), 2);
        assert_eq!(reg.num_overlapping_sections(), 4);
    }
}