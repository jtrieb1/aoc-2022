use crate::{util::{AOCSolution, parse_lines_into, read_input_to_str}, SResult};
use std::str::FromStr;

solution!(Day 15 => SignalMap);

pub struct SignalMap {
    readings: Vec<Reading>,
}

impl AOCSolution for SignalMap {
    fn load_from(input_file_path: &str) -> SResult<Box<Self>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        format!("{}", self.count_impossible_cells_in_row(2000000))
    }
    
    fn part_2(&mut self) -> String {
        format!("{:?}", self.get_tuning_freq(0, 4000000, 0, 4000000))
    }
    
    
}

impl SignalMap {
    pub fn new_from_file(input_path: &str) -> SResult<Self> {
        let input_str = read_input_to_str(input_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn new_from_str(input_str: &str) -> SResult<Self> {
        let readings = parse_lines_into::<Reading>(input_str)?;
        Ok(Self {
            readings,
        })
    }
    
    fn get_tuning_freq(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i64 {
        if let Some((x, y)) = self.find_unscanned(min_x, max_x, min_y, max_y) {
            // An abundance of caution
            return x as i64 * 4000000 + y as i64
        }
        -1 // Not returning Option here for printing reasons.
    }
    
    fn find_unscanned(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Option<(i32, i32)> {
        for row in min_y ..= max_y {
            if let Some(cell) = self.find_unscanned_cells_in_row_range(row, min_x, max_x) {
                return Some((cell, row));
            }
        }
        None
    }
    
    fn find_unscanned_cells_in_row_range(&self, row: i32, min_x: i32, max_x: i32) -> Option<i32> {
        let ranges = self.readings.iter()
            .map(|r| r.get_unscanned_in_row_range(row, min_x, max_x))
            .collect::<Vec<Vec<CellRange>>>();
            
        let mut current_coll = ranges.first().unwrap().clone();
        for coll in ranges.iter().skip(1) {
            let mut new_coll = Vec::new();
            for range in current_coll.iter() {
                for rr in coll.iter() {
                    if let Some(u) = rr.intersect(range) {
                        new_coll.push(u);
                    }
                }
            }
            if new_coll.len() == 0 { return None; }
            current_coll = new_coll;
        }
        
        Some(current_coll.first().unwrap().0)
    }
    
    fn count_impossible_cells_in_row(&self, row: i32) -> usize {
        let mut invalid_ranges = self.readings
            .iter()
            .map(|r| r.get_invalid_cells_in_row_range(row))
            .flatten()
            .collect::<Vec<CellRange>>();
            
        invalid_ranges.sort_unstable();
        let mut res: Vec<CellRange> = Vec::new();
        for range in invalid_ranges.iter() {
            if let Some(r) = res.pop() {
                let u = r.union(&range);
                res.extend(u)
            } else {
                res.push(range.clone())
            }
        }
        res.iter().map(|r| r.count()).sum()
        
    }
}

fn manhattan_distance(start: &(i32, i32), end: &(i32, i32)) -> i32 {
    (end.0 - start.0).abs() + (end.1 - start.1).abs()
}

struct Reading {
    sensor: Sensor,
    beacon: Beacon,
    distance: i32,
}

impl FromStr for Reading {
    type Err = ReadingParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(":").collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(ReadingParseError("Invalid number of sections in reading"));
        }
        let sensor = sections[0].parse::<Sensor>().map_err(|_| ReadingParseError("Invalid sensor data"))?;
        let beacon = sections[1].parse::<Beacon>().map_err(|_| ReadingParseError("Invalid beacon data"))?;
        let distance = manhattan_distance(&(sensor.0, sensor.1), &(beacon.0, beacon.1));
        Ok(Self {
            sensor,
            beacon,
            distance,
        })
    }
}

impl Reading {
    
    pub fn get_invalid_cells_in_row_range(&self, row: i32) -> Vec<CellRange> {
        let dy = (self.sensor.1 - row).abs();
        let x0 = self.sensor.0;
        let d0 = self.distance;
        if dy > self.distance {
            return vec![];
        }
        let range = CellRange(x0 + dy - d0, x0 + d0 - dy);
        if row == self.beacon.1 {
            return range.without(self.beacon.0);
        }
        vec![range]
    }
    
    pub fn get_unscanned_in_row_range(&self, row: i32, min_x: i32, max_x: i32) -> Vec<CellRange> {
        let dy = (self.sensor.1 - row).abs();
        let x0 = self.sensor.0;
        let d0 = self.distance;
        if dy >= d0 { return vec![CellRange(min_x, max_x)]; }
        let left = x0 + dy - d0 - 1;
        let right = x0 + d0 - dy + 1;
        if left >= min_x && right <= max_x {
            vec![CellRange(min_x, left), CellRange(right, max_x)]
        } else if left < min_x && right > max_x {
            vec![]
        } else if left < min_x {
            vec![CellRange(right, max_x)]
        } else if right > max_x {
            vec![CellRange(min_x, left)]
        } else {
            vec![]
        }
    }
}

custom_error!(ReadingParseError);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct CellRange(i32, i32);

impl CellRange {
    pub fn union(&self, other: &CellRange) -> Vec<CellRange> {
        if self.contains(other) {
            vec![self.clone()]
        } else if other.contains(self) {
            vec![other.clone()]
        } else if !self.overlaps(other) {
            vec![self.clone(), other.clone()]
        } else {
            if self.1 >= other.0 && other.0 >= self.0 {
                vec![CellRange(self.0, other.1)]
            } else {
                vec![CellRange(other.0, self.1)]
            }
        }
    }
    
    fn overlaps(&self, other: &CellRange) -> bool {
        (self.1 >= other.0 && other.0 >= self.0) ||
        (self.0 <= other.1 && other.1 <= self.1) ||
        self.contains(other) || other.contains(self)
    }
    
    fn contains(&self, other: &CellRange) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    
    pub fn without(&self, point: i32) -> Vec<CellRange> {
        if point < self.0 || point > self.1 {
            vec![self.clone()]
        }
        else if point == self.0 {
            vec![CellRange(self.0 + 1, self.1)]
        }
        else if point == self.1 {
            vec![CellRange(self.0, self.1 - 1)]
        }
        else {
            vec![CellRange(self.0, point - 1), CellRange(point + 1, self.1)]
        }
    }
    
    pub fn intersect(&self, other: &CellRange) -> Option<CellRange> {
        if !self.overlaps(other) {
            return None;
        }
        Some(CellRange(i32::max(self.0, other.0), i32::min(self.1, other.1)))
    }
    
    pub fn count(&self) -> usize {
        (self.1 - self.0) as usize + 1
    }
}

struct Sensor(i32, i32);
struct Beacon(i32, i32);

impl Into<(i32, i32)> for Sensor {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl Into<(i32, i32)> for Beacon {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl FromStr for Sensor {
    type Err = SensorParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(coord_string) = s.split("x=").nth(1) {
            let mut parsed = Vec::new();
            for s_coord in coord_string.split(", y=") {
                if let Ok(coord) = s_coord.parse::<i32>() {
                    parsed.push(coord);
                }
            }
            if parsed.len() != 2 {
                return Err(SensorParseError("Invalid number of coords for sensor"));
            }
            Ok(Self(parsed[0], parsed[1]))
        } else {
            Err(SensorParseError("Invalid sensor string"))
        }
    }
}

custom_error!(SensorParseError);

impl FromStr for Beacon {
    type Err = BeaconParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(coord_string) = s.split("x=").nth(1) {
            let mut parsed = Vec::new();
            for s_coord in coord_string.split(", y=") {
                if let Ok(coord) = s_coord.parse::<i32>() {
                    parsed.push(coord);
                }
            }
            if parsed.len() != 2 {
                return Err(BeaconParseError("Invalid number of coords for sensor"));
            }
            Ok(Self(parsed[0], parsed[1]))
        } else {
            Err(BeaconParseError("Invalid beacon string"))
        }
    }
}

custom_error!(BeaconParseError);

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given() {
        let input_str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        
        let map = SignalMap::new_from_str(input_str).expect("Failed to parse");
        assert_eq!(map.count_impossible_cells_in_row(10), 26);
        assert_eq!(map.get_tuning_freq(0, 20, 0, 20), 56000011);
    }
}