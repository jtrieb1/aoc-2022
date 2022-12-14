use crate::util::{AOCSolution, SResult, parse_lines_into, read_input_to_str};
use std::{collections::HashSet, str::FromStr};

solution!(Day 14 => Cave);

#[derive(Debug)]
pub struct Cave {
    blocked: HashSet<(usize, usize)>,
    sand: Vec<Sand>,
    max_y: usize,
    floor: usize
}

impl AOCSolution for Cave {
    fn load_from(input_file_path: &str) -> SResult<Box<Self>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        self.run_until_complete(false);
        format!("{}", self.count_grains())
    }
    
    fn part_2(&mut self) -> String {
        self.reset();
        self.run_until_complete(true);
        format!("{}", self.count_grains())
    }
}

impl Cave {
    pub fn new_from_file(input_path: &str) -> SResult<Self> {
        let input_str = read_input_to_str(input_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn new_from_str(input_str: &str) -> SResult<Self> {
        let rocks = parse_lines_into::<Rock>(input_str)?;
        let mut blocked = HashSet::new();
        let mut max_y = 0;
        for rock in rocks.iter() {
            rock.populate_blocked(&mut blocked);
            max_y = usize::max(max_y, rock.max_y());
        }
        Ok(Self {
            blocked,
            sand: Vec::new(),
            max_y,
            floor: 2 + max_y
        })
    }
    
    pub fn run_until_complete(&mut self, with_floor: bool) {
        if !with_floor {
            while self.simulate_sand_fall() {}
        } else {
            while self.simulate_sand_fall_floor() {}
        }
    }
    
    pub fn simulate_sand_fall(&mut self) -> bool {
        // Returns false if grain falls off map
        let mut grain = Sand::new(500, 0);
        while grain.did_move(&self.blocked) {
            if grain.position().1 > self.max_y {
                return false;
            }
        }
        self.sand.push(grain.clone());
        self.blocked.insert(grain.position());
        return true;
    }
    
    pub fn simulate_sand_fall_floor(&mut self) -> bool {
        // Returns false once grain stops at origin
        let mut grain = Sand::new(500, 0);
        while grain.did_move_with_floor(&self.blocked, self.floor) {}
        self.sand.push(grain.clone());
        self.blocked.insert(grain.position());
        grain.position() != (500, 0)
    }
    
    pub fn count_grains(&self) -> usize {
        self.sand.len()
    }
    
    pub fn reset(&mut self) {
        for grain in self.sand.iter() {
            self.blocked.remove(&grain.position());
        }
        self.sand = Vec::new();
    }
}

#[derive(Debug)]
struct Rock {
    cells: Vec<(usize, usize)>
}

impl FromStr for Rock {
    type Err = RockParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.split(" -> ");
        let mut cells = Vec::new();
        let mut coords = Vec::new();
        for segment in segments {
            let xy = segment.split(',').collect::<Vec<&str>>();
            if xy.len() != 2 {
                return Err(RockParseError("Invalid segment length, expected X,Y"));
            }
            let x = xy[0].parse::<usize>().map_err(|_| RockParseError("Invalid number"))?;
            let y = xy[1].parse::<usize>().map_err(|_| RockParseError("Invalid number"))?;
            coords.push((x, y));
        }
        for endpoints in coords.windows(2) {
            let mut c1 = endpoints[0];
            let c2 = endpoints[1];
            while c1 != c2 {
                cells.push(c1);
                let dx = (c2.0 as i32 - c1.0 as i32).signum();
                let dy = (c2.1 as i32 - c1.1 as i32).signum();
                c1 = ((c1.0 as i32 + dx) as usize, (c1.1 as i32 + dy) as usize);
            }
            cells.push(c2)
        }
        cells.dedup();
        Ok(Self { cells })
    }
}

custom_error!(RockParseError);

impl Rock {
    pub fn populate_blocked(&self, map: &mut HashSet<(usize, usize)>) {
        for cell in self.cells.iter() {
            map.insert(cell.clone());
        }
    }
    
    pub fn max_y(&self) -> usize {
        let mut max_y = 0;
        for cell in self.cells.iter() {
            max_y = usize::max(max_y, cell.1);
        }
        max_y
    }
}

#[derive(Debug, Clone)]
struct Sand {
    cell: (usize, usize)
}

impl Sand {
    pub fn new(x: usize, y: usize) -> Self {
        Self { cell: (x, y) }
    }
    
    pub fn did_move(&mut self, blocks: &HashSet<(usize, usize)>) -> bool {
        let possible_targets = vec![
            (self.cell.0,     self.cell.1 + 1),
            (self.cell.0 - 1, self.cell.1 + 1),
            (self.cell.0 + 1, self.cell.1 + 1)
        ];
        for target in possible_targets {
            if blocks.contains(&target) {
                continue;
            }
            self.cell = target;
            return true;
        }
        false
    }
    
    pub fn did_move_with_floor(&mut self, blocks: &HashSet<(usize, usize)>, floor: usize) -> bool {
        if self.cell.1 + 1 == floor {
            return false;
        }
        self.did_move(blocks)
    }
    
    pub fn position(&self) -> (usize, usize) {
        self.cell.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given() {
        let input_str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
        ";
        
        let mut cave = Cave::new_from_str(input_str).expect("Failed to parse rocks");
        assert_eq!(cave.part_1(), "24");
        assert_eq!(cave.part_2(), "93");
    }
}