use crate::util::*;

solution!(Day 1 => ElfManifest);

pub struct ElfManifest {
    inventories: Vec<ElfInventory>,
}

impl AOCSolution for ElfManifest {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>> where Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        self.get_max_calories().to_string()
    }

    fn part_2(&mut self) -> String {
        self.get_top_n_calorie_sum(3).to_string()
    }
}

impl ElfManifest {
    pub fn new_from_file(input_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_path)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sections = convert_str_to_sections(input)?;
        Self::new(sections)
    }

    pub fn new(input_sections: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut inventories = Vec::new();
        for section in input_sections.iter() {
            let inv = ElfInventory::new(&section)?;
            inventories.push(inv);
        }
        Ok(Self { inventories })
    }

    pub fn get_max_calories(&self) -> u32 {
        self.get_all_calorie_totals()
            .into_iter()
            .max()
            .unwrap_or(0)
    }

    pub fn get_top_n_calorie_sum(&self, n: usize) -> u32 {
        let mut cal_vec: Vec<u32> = self.get_all_calorie_totals();
        cal_vec.sort_unstable();
        cal_vec.iter().rev().take(n).sum()
    }

    fn get_all_calorie_totals(&self) -> Vec<u32> {
        self
            .inventories
            .iter()
            .map(|inv| inv.get_calories())
            .collect()
    }
}

struct ElfInventory {
    entries: Vec<u32>,
}

impl ElfInventory {
    pub fn new(repr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let entries = parse_lines_into::<u32>(repr)?;

        Ok(Self { entries })
    }

    pub fn get_calories(&self) -> u32 {
        self.entries.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let test_input = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        ";

        let manifest = ElfManifest::new_from_str(test_input).expect("Failed to parse manifest.");
        let result = manifest.get_max_calories();
        assert!(result == 24000);
    }
}
