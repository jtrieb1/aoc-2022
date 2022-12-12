use std::{str::FromStr, fmt::Debug};

use crate::util::{convert_str_to_sections, read_input_to_str, AOCSolution};

solution!(Day 11 => KeepAway);

pub struct KeepAway {
    input: String,
    monkeys: Vec<Monkey>,
    modulus: usize,
}

impl AOCSolution for KeepAway {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        self.run_n_rounds(20, true);
        format!("{}", self.calculate_monkey_business())
    }
    
    fn part_2(&mut self) -> String {
        self.reload().unwrap();
        self.run_n_rounds(10000, false);
        format!("{}", self.calculate_monkey_business())
    }
}

impl KeepAway {
    pub fn new_from_file(input_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let input_str = self.input.clone();
        let sections = convert_str_to_sections(&input_str, true)?;
        let monkeys = sections.iter().map(|s| s.parse::<Monkey>().unwrap()).collect::<Vec<Monkey>>();
        self.monkeys = monkeys;
        Ok(())
    }
    
    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sections = convert_str_to_sections(input_str, true)?;
        let monkeys = sections.iter().map(|s| s.parse::<Monkey>().unwrap()).collect::<Vec<Monkey>>();
        let modulus = monkeys.iter().map(|m| m.test_val).product();
        Ok(Self {
            input: input_str.to_string(),
            monkeys,
            modulus
        })
    }
    
    pub fn run_n_rounds(&mut self, n: usize, reduce: bool) {
        for _ in 0..n {
            self.run_round(reduce);
        }
    }
    
    fn run_round(&mut self, reduce: bool) {
        for monkey_idx in 0..self.monkeys.len() {
            let insp_count = self.handle_monkey_actions(monkey_idx, reduce);
            self.monkeys[monkey_idx].items = Vec::new();
            self.monkeys[monkey_idx].inspection_count += insp_count;
        }
    }
    
    fn handle_monkey_actions(&mut self, monkey_idx: usize, reduce: bool) -> usize{
        let monkey = &self.monkeys[monkey_idx];
        let transfers = monkey.execute_turn(reduce, self.modulus);
        let num_inspections = transfers.len();
        for transfer in transfers {
            let target_monkey = &mut self.monkeys[transfer.target];
            target_monkey.items.push(transfer.item);
        }
        num_inspections
    }
    
    pub fn calculate_monkey_business(&self) -> usize {
        let mut counts = self.monkeys.iter()
            .map(|m| m.inspection_count)
            .collect::<Vec<usize>>();
        counts.sort_unstable();
        counts.iter().rev().take(2).product()
    }
}

struct Monkey {
    items: Vec<usize>,
    inspection_count: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    test_val: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl FromStr for Monkey {
    type Err = MonkeyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // Ignore monkey number
        let starting_items = Monkey::parse_starting_items(lines.next().unwrap())?;
        let operation = Monkey::parse_operation(lines.next().unwrap())?;
        let (test, test_val) = Monkey::parse_test(lines.next().unwrap())?;
        let true_monkey = Monkey::parse_target_monkey(lines.next().unwrap());
        let false_monkey = Monkey::parse_target_monkey(lines.next().unwrap());
        Ok(Self {
            items: starting_items,
            inspection_count: 0,
            operation,
            test,
            test_val,
            true_monkey,
            false_monkey
        })
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Monkey: {:?}, Inspections: {}", self.items, self.inspection_count))
    }
}

impl Monkey {
    fn parse_starting_items(s: &str) -> Result<Vec<usize>, MonkeyParseError> {
        let sections = s.split(':').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(MonkeyParseError("Invalid starting items line"));
        }
        Ok(sections[1]
            .split(',')
            .map(|e| {
                e
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| MonkeyParseError("Invalid item #"))
            })
            .filter(|v| v.is_ok())
            .map(|v| v.unwrap())
            .collect::<Vec<usize>>())
    }
    
    fn parse_operation(s: &str) -> Result<Box<dyn Fn(usize) -> usize>, MonkeyParseError> {
        let sections = s.split(':').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(MonkeyParseError("Invalid operation line"));
        }
        if let Some((_, remaining)) = sections[1].split_once("old") {
            let op_def = remaining.trim().split_whitespace().collect::<Vec<&str>>();
            if op_def.len() != 2 {
                return Err(MonkeyParseError("Invalid number of rhs args"));
            }
            let reflexive = op_def.last().unwrap() == &"old";
            let op_val = op_def[1].parse::<usize>().unwrap_or(0);
            match (op_def[0], reflexive) {
                ("+", false) => Ok(Box::new(move |item| item + op_val)),
                ("+", true)  => Ok(Box::new(move |item| item + item)),
                ("*", false) => Ok(Box::new(move |item| item * op_val)),
                ("*", true)  => Ok(Box::new(move |item| item * item)),
                ("-", false) => Ok(Box::new(move |item| item - op_val)),
                ("-", true)  => Ok(Box::new(move |_| 0)),
                ("/", false) => Ok(Box::new(move |item| item / op_val)),
                ("/", true)  => Ok(Box::new(move |_| 1)),
                _ => Err(MonkeyParseError("Invalid or undefined operation"))
            }
        } else {
            return Err(MonkeyParseError("Invalid op section line"));
        }
    }
    
    fn parse_test(s: &str) -> Result<(Box<dyn Fn(usize) -> bool>, usize), MonkeyParseError> {
        let sections = s.split(':').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(MonkeyParseError("Invalid operation line"));
        }
        let test_num = 
            sections[1]
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .map_err(|_| MonkeyParseError("Invalid test number"))?;
        Ok((Box::new(move |item| item % test_num == 0), test_num))
    }
    
    fn parse_target_monkey(s: &str) -> usize {
        let target_val = 
            s
                .split(' ')
                .last()
                .map(|s| s.parse::<usize>().unwrap_or(usize::MAX))
                .unwrap();
        target_val
    }
    
    pub fn execute_turn(&self, reduce: bool, modulus: usize) -> Vec<MonkeyTransfer> {
        let mut transfers = Vec::new();
        for item in self.items.iter() {
            let mut inspect_result = (self.operation)(*item);
            if reduce {
                inspect_result /= 3;
            } else {
                // Modulus is product of all monkey moduli to prevent test value changes
                // k % p == (k % r*p) % p for natural numbers k, r, and prime p
                inspect_result %= modulus;
            }
            let check = (self.test)(inspect_result);
            transfers.push(MonkeyTransfer {
                item: inspect_result,
                target: if check { self.true_monkey } else { self.false_monkey }
            })
        }
        transfers
    }
}

custom_error!(MonkeyParseError);

struct MonkeyTransfer {
    item: usize,
    target: usize
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given() {
        let input_str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        ";
    
        let mut keepaway = KeepAway::new_from_str(input_str).expect("Unable to parse");
        assert_eq!(keepaway.monkeys.len(), 4);
        keepaway.run_n_rounds(20, true);
        assert_eq!(keepaway.calculate_monkey_business(), 10605);
        keepaway.reload().unwrap();
        keepaway.run_n_rounds(10000, false);
        assert_eq!(keepaway.calculate_monkey_business(), 2713310158);
    }
}