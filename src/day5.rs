use crate::util::{read_input_to_str, parse_lines_into, convert_str_to_sections, AOCSolution};
use std::str::FromStr;

solution!(Day 5 => CrateTowers);

pub struct CrateTowers {
    src: String,
    stacks: Vec<CrateStack>,
    instructions: Vec<CraneInstruction>
}

impl AOCSolution for CrateTowers {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
        where
            Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        self.simulate_cratemover9000();
        let tops = self.get_top_of_stacks();
        let tops = tops.iter().map(|t| t.unwrap().0).collect::<String>();

        format!("{}", tops)
    }

    fn part_2(&mut self) -> String {
        self.reset();
        self.simulate_cratemover9001();
        let tops = self.get_top_of_stacks();
        let tops = tops.iter().map(|t| t.unwrap().0).collect::<String>();

        format!("{}", tops)
    }
}

impl CrateTowers {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input = read_input_to_str(input_file_path, false)?;
        Self::new_from_str(&input)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sections = convert_str_to_sections(input_str, false)?;
        let crate_diagram = sections.first().expect("Should have two sections");
        let instructions = parse_lines_into::<CraneInstruction>(&sections[1])?;
        let stacks = Self::parse_crate_diagram(&crate_diagram)?;
        Ok(Self {
            src: input_str.to_string(),
            stacks,
            instructions
        })
    }

    pub fn reset(&mut self) {
        let sections = convert_str_to_sections(&self.src, false).expect("Worked on instantiation");
        let crate_diagram = sections.first().expect("Should have diagram");
        let stacks = Self::parse_crate_diagram(&crate_diagram).expect("Worked on instantiation");
        self.stacks = stacks;
    }

    fn parse_crate_diagram(diagram: &str) -> Result<Vec<CrateStack>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        let lines = diagram.lines();
        // Every representative element is 3 chars with 1 char spacing between
        for line in lines {
            let line_chars = line.chars().into_iter().collect::<Vec<char>>();
            let blocks = line_chars.chunks(4);
            for (idx, block) in blocks.enumerate() {
                if result.len() <= idx {
                    result.push(CrateStack::new());
                }
                let cr_repr: String = block.into_iter().collect();
                let cr_repr = cr_repr.trim();
                if let Ok(cr) = cr_repr.parse::<Crate>() {
                    result[idx].load(cr);
                }
            }
        }

        Ok(result)
    }

    fn simulate_cratemover9000(&mut self) {
        let instructions = self.instructions.clone();
        for inst in instructions.iter() {
            self.cratemover9000_execute(inst);
        }
    }

    fn cratemover9000_execute(&mut self, inst: &CraneInstruction) {
        let mut crates = self.get_top_n_of_stack(inst.num, inst.src);
        crates.reverse();
        self.stacks[inst.target as usize].extend(&mut crates);
    }

    fn simulate_cratemover9001(&mut self) {
        let instructions = self.instructions.clone();
        for inst in instructions.iter() {
            self.cratemover9001_execute(inst);
        }
    }

    fn cratemover9001_execute(&mut self, inst: &CraneInstruction) {
        let mut crates = self.get_top_n_of_stack(inst.num, inst.src);
        self.stacks[inst.target as usize].extend(&mut crates);
    }

    fn get_top_n_of_stack(&mut self, n: u32, stack: u32) -> Vec<Crate> {
        let stack = &mut self.stacks[stack as usize];
        stack.pop_last_n(n as usize)
    }

    fn get_top_of_stacks(&self) -> Vec<Option<&Crate>> {
        let mut result = Vec::new();
        for stack in self.stacks.iter() {
            result.push(stack.top())
        }
        result
    }
}

struct CrateStack {
    crates: Vec<Crate>
}

impl CrateStack {
    pub fn new() -> Self {
        Self {
            crates: Vec::new()
        }
    }

    pub fn load(&mut self, c: Crate) {
        self.crates.insert(0, c);
    }

    pub fn pop_last_n(&mut self, n: usize) -> Vec<Crate> {
        let start_idx = self.crates.len() - n;
        self.crates.drain(start_idx..).collect()
    }

    pub fn top(&self) -> Option<&Crate> {
        self.crates.last()
    }

    pub fn extend(&mut self, crates: &mut Vec<Crate>) {
        self.crates.append(crates);
    }
}


#[derive(Debug)]
struct Crate(char);

impl FromStr for Crate {
    type Err = CrateParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(CrateParseError("Invalid crate length"));
        }
        Ok(Self(s.chars().collect::<Vec<char>>()[1]))
    }
}

#[derive(Clone, Copy)]
struct CraneInstruction {
    num: u32,
    src: u32,
    target: u32
}

impl FromStr for CraneInstruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: [&str; 6] = s.split(' ').collect::<Vec<&str>>().try_into().map_err(|_| InstructionParseError("Invalid instruction length"))?;
        let num = sections[1].parse::<u32>().map_err(|_| InstructionParseError("Crate count not a number."))?;
        let src = sections[3].parse::<u32>().map_err(|_| InstructionParseError("Stack num not a number."))?;
        let target = sections[5].parse::<u32>().map_err(|_| InstructionParseError("Stack num not a number."))?;

        // src and target are 1-indexed in input, 0-indexed in code.

        Ok(Self {
            num,
            src: src - 1,
            target: target - 1
        })
    }
}

custom_error!(CrateParseError);
custom_error!(InstructionParseError);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

        let mut towers = CrateTowers::new_from_str(input).expect("Failed to parse");
        assert_eq!(towers.stacks.len(), 3);
        assert_eq!(towers.instructions.len(), 4);
        assert_eq!(towers.part_1(), "CMZ");
        assert_eq!(towers.part_2(), "MCD");
    }
}