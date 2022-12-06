use crate::util::{read_input_to_str, AOCSolution};

solution!(Day 6 => CommSystem);

pub struct CommSystem {
    stream: String,
}

impl AOCSolution for CommSystem {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        format!("{}", self.scan_for_signal_start())
    }

    fn part_2(&mut self) -> String {
        format!("{}", self.scan_for_message_start())
    }
}

impl CommSystem {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            stream: input_str.to_string(),
        })
    }

    pub fn scan_for_signal_start(&self) -> usize {
        self.find_first_unique_substream_of_size(4)
    }

    pub fn scan_for_message_start(&self) -> usize {
        self.find_first_unique_substream_of_size(14)
    }

    /*
    // This solution is conceptually simpler, but uses a lot of iterations and sorts,
    // which are nice to avoid.

    fn find_first_unique_substream_of_size(&self, size: usize) -> usize {
        for (idx, packet) in self.stream.as_bytes().windows(size).enumerate() {
            let mut  pvec = packet.to_vec();
            pvec.sort_unstable();
            pvec.dedup();
            if pvec.len() == size {
                return idx + size;
            }
        }
        0
    }

    */

    fn find_first_unique_substream_of_size(&self, size: usize) -> usize {
        let mut scanner = Scanner::new(size);
        scanner.scan(self.stream.as_bytes())
    }
}

struct Scanner {
    buf_size: usize,
    buffer: Vec<u8>,
    counts: std::collections::HashMap<u8, u8>,
    position: usize,
}

impl Scanner {
    pub fn new(size: usize) -> Self {
        Self {
            buf_size: size,
            buffer: Vec::new(),
            counts: std::collections::HashMap::new(),
            position: 0,
        }
    }
    
    pub fn scan(&mut self, bytestream: &[u8]) -> usize {
        for byte in bytestream.iter() {
            self.consume(*byte);
            if self.check_unique() {
                return self.position;
            }
        }
        0
    }

    fn consume(&mut self, next: u8) {
        if self.buffer.len() == self.buf_size {
            let cycling = self.buffer.remove(0);
            self.remove(cycling);
        }
        self.append(next);
        self.position += 1;
    }
    
    fn remove(&mut self, byte: u8) {
        self.counts.entry(byte).and_modify(|v| *v -= 1);
        if self.counts[&byte] == 0 {
            self.counts.remove(&byte);
        }
    }

    fn append(&mut self, next: u8) {
        self.buffer.push(next);
        self.counts.entry(next).and_modify(|v| *v += 1).or_insert(1);
    }

    fn check_unique(&self) -> bool {
        self.counts.len() == self.buf_size 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        given: &'static str,
        part1: &'static str,
        part2: &'static str,
    }

    impl TestCase {
        pub fn new(given: &'static str, part1: &'static str, part2: &'static str) -> Self {
            Self {
                given,
                part1,
                part2,
            }
        }

        pub fn execute(&self) {
            let mut system = CommSystem::new_from_str(self.given).expect("Invalid input");
            assert_eq!(system.part_1(), self.part1);
            assert_eq!(system.part_2(), self.part2);
        }
    }

    fn test_data() -> Vec<TestCase> {
        vec![
            TestCase::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb",    "7",  "19"),
            TestCase::new("bvwbjplbgvbhsrlpgdmjqwftvncz",      "5",  "23"),
            TestCase::new("nppdvjthqldpwncqszvftbrmjlhg",      "6",  "23"),
            TestCase::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", "29"),
            TestCase::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",  "11", "26"),
        ]
    }

    #[test]
    fn given_test() {
        for test in test_data().iter() {
            test.execute();
        }
    }
}
