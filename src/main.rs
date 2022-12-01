mod util;
mod day1;

pub use util::*;
use day1::ElfManifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manifest = ElfManifest::load_from("input/day1.txt")?;
    print_solution(1, &mut manifest);
    Ok(())
}

fn print_solution(day: u8, sol: &mut dyn AOCSolution) {
    println!("Day {}:", day);
    println!("Part 1: {}", sol.part_1());
    println!("Part 2: {}", sol.part_2());
    println!();
}