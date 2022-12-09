use std::{str::FromStr, collections::HashSet};

use crate::util::{AOCSolution, parse_lines_into, read_input_to_str};

solution!(Day 9 => Simulator);

pub struct Simulator {
    head: HeadPosition,
    tail: Vec<TailPosition>,
    instructions: Vec<Instruction>
}

impl AOCSolution for Simulator {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        self.run();
        format!("{}", self.get_unique_tail_positions())
    }
    
    fn part_2(&mut self) -> String {
        self.reset();
        self.with_tails(9);
        self.run();
        format!("{}", self.get_unique_tail_positions())
    }
}

impl Simulator {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions = parse_lines_into::<Instruction>(input_str)?;
        let head = HeadPosition::new(0,0);
        let tail = vec![TailPosition::new(0, 0)];
        Ok(Self {
            instructions,
            head,
            tail
        })
    }
    
    pub fn run(&mut self) {
        for inst in self.instructions.iter() {
            for _ in 0..inst.steps {
                self.head.step(&inst.direction);
                self.tail.first_mut().unwrap().update(&self.head.clone().into());
                for i in 1..self.tail.len() {
                    let prev = self.tail[i - 1].clone();
                    self.tail[i].update(&prev.into());
                }
            }
        }
    }
    
    pub fn with_tails(&mut self, tail_num: usize) {
        if self.tail.len() >= tail_num { return; }
        for _ in 0..tail_num - self.tail.len() {
            self.tail.push(TailPosition::new(0,0));
        }
    }
    
    pub fn reset(&mut self) {
        self.head.position = (0,0);
        for tail in self.tail.iter_mut() {
            tail.position = (0,0);
        }
    }
    
    pub fn get_unique_tail_positions(&self) -> usize {
        self.tail.last().unwrap().visited.len()
    }
}

#[derive(Clone)]
struct HeadPosition {
    position: (i32, i32)
}

impl Into<(i32, i32)> for HeadPosition {
    fn into(self) -> (i32, i32) {
        self.position
    }
}

impl HeadPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: (x, y)
        }
    }
    
    pub fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::R => self.position.0 += 1,
            Direction::L => self.position.0 -= 1,
            Direction::U => self.position.1 += 1,
            Direction::D => self.position.1 -= 1
        }
    }
}

#[derive(Clone)]
struct TailPosition {
    position: (i32, i32),
    visited: HashSet<(i32, i32)>
}

impl TailPosition {
    pub fn new(x: i32, y: i32) -> Self {
        let mut visited = HashSet::new();
        visited.insert((x, y));
        Self {
            position: (x, y),
            visited
        }
    }
    
    pub fn update(&mut self, head: &(i32, i32)) {
        if self.is_touching(head) { return; }
        let to_move = self.direction_towards(head);
        self.position.0 += to_move.0;
        self.position.1 += to_move.1;
        self.visited.insert(self.position);
    }
    
    fn is_touching(&self, head: &(i32, i32)) -> bool {
        (self.position.0 - head.0).abs() <= 1 && (self.position.1 - head.1).abs() <= 1
    }
    
    fn direction_towards(&self, head: &(i32, i32)) -> (i32, i32) {
        let x = (head.0 - self.position.0).signum();
        let y = (head.1 - self.position.1).signum();
        (x, y)
    }
}

impl Into<(i32, i32)> for TailPosition {
    fn into(self) -> (i32, i32) {
        self.position
    }
}

struct Instruction {
    direction: Direction,
    steps: usize
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(' ').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(InstructionParseError("Invalid instruction length"));
        }
        let direction = match sections[0] {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            _ => Direction::D
        };
        let steps = sections[1].parse::<usize>().map_err(|_| InstructionParseError("Invalid steps"))?;
        Ok(Self {
            direction,
            steps
        })
    }
}

custom_error!(InstructionParseError);

enum Direction {
    R,
    L,
    U,
    D
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given_test() {
        let input_str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let mut sim = Simulator::new_from_str(input_str).expect("Failed to parse");
        assert_eq!(sim.part_1(), "13");
        assert_eq!(sim.part_2(), "1");
    }
}