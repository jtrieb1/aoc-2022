use crate::{AOCSolution, read_input_to_str, parse_lines_into};
use std::str::FromStr;

solution!(Day 10 => CPU);

pub struct CPU {
    clock: u32,
    register: i32,
    instructions: Vec<CPUInstruction>,
    program_counter: usize,
    signal_snapshots: Vec<i32>,
    crt: CRT
}

impl AOCSolution for CPU {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        self.run();
        format!("{}", self.get_relevant_sum())
    }
    
    fn part_2(&mut self) -> String {
        format!("\n{}", self.show_screen())
    }
}

impl CPU {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions = parse_lines_into::<CPUInstruction>(input_str)?;
        Ok(Self {
            clock: 1,
            register: 1,
            instructions,
            program_counter: 0,
            signal_snapshots: Vec::new(),
            crt: CRT::new()
        })
    }
    
    pub fn run(&mut self) {
        while self.program_counter < self.instructions.len() {
            self.tick();
        }
    }
    
    fn tick(&mut self) {
        let instr = &self.instructions[self.program_counter];
        match instr {
            CPUInstruction::Noop => self.noop(),
            CPUInstruction::AddX(v) => self.addx(*v)
        }
        self.program_counter += 1;
    }
    
    fn noop(&mut self) {
        self.take_snapshot();
        self.increment_clock();
    }
    
    fn addx(&mut self, v: i32) {
        self.take_snapshot();
        self.increment_clock();
        self.take_snapshot();
        self.increment_clock();
        self.register += v;
    }
    
    fn increment_clock(&mut self) {
        self.crt.tick(self.register);
        self.clock += 1;
    }
    
    fn take_snapshot(&mut self) {
        self.signal_snapshots.push(self.clock as i32 * self.register);
    }
    
    fn get_relevant_sum(&self) -> i32 {
        self.signal_snapshots
            .iter()
            .skip(19)
            .step_by(40)
            .take(6)
            .sum()
    }
    
    fn show_screen(&self) -> String {
        self.crt.render()
    }
}

enum CPUInstruction {
    Noop,
    AddX(i32)
}

impl FromStr for CPUInstruction {
    type Err = CPUInstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(' ').collect::<Vec<&str>>();
        if sections.len() > 2 || sections.len() == 0 {
            return Err(CPUInstructionParseError("Invalid instruction length."));
        }
        match sections[0] {
            "addx" => Ok(
                CPUInstruction::AddX(
                    sections[1].parse::<i32>().map_err(|_| CPUInstructionParseError("Invalid number"))?
                    )
                ),
            _ => Ok(CPUInstruction::Noop)
        }
    }
}

custom_error!(CPUInstructionParseError);

struct CRT {
    pixels: Vec<bool>,
    clock: u32,
    sprite_position: i32
}

impl CRT {
    pub fn new() -> Self {
        Self {
            pixels: Vec::new(),
            clock: 1,
            sprite_position: 1
        }
    }
    
    pub fn tick(&mut self, register: i32) {
        self.shift_sprite(register);
        self.pixels.push(self.is_in_sprite(self.clock as usize));
        self.clock += 1;
    }
    
    fn shift_sprite(&mut self, amount: i32) {
        self.sprite_position = amount
    }
    
    fn is_in_sprite(&self, pixel: usize) -> bool {
        (self.sprite_position - (pixel as i32 - 1) % 40).abs() <= 1
    }
    
    pub fn render(&self) -> String {
        self.pixels
            .chunks(40)
            .map(|line| {
                line.iter()
                    .map(|&b| if b { "#" } else { "." })
                    .collect::<String>()
                + "\n"
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given_test() {
        let input_str = "
        addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
        ";
        let mut cpu = CPU::new_from_str(input_str).expect("Unable to parse");
        cpu.run();
        assert_eq!(cpu.get_relevant_sum(), 13140);
        println!("{}", cpu.show_screen()); // Visual check
    }
}