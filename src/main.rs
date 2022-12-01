mod util;
mod day1;

pub use util::*;
use day1::ElfManifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1:");
    let manifest = ElfManifest::new(&read_input_to_str("input/day1.txt")?)?;
    println!("Part 1: {}", manifest.get_max_calories());
    println!("Part 2: {}", manifest.get_top_n_calorie_sum(3));
    Ok(())
}
