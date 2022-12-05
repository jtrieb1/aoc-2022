#[macro_use]
mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions: Vec<Box<dyn AOCSolution>> = vec![
        day1::solution()?,
        day2::solution()?,
        day3::solution()?,
        day4::solution()?,
        day5::solution()?
    ];
    print_solutions(solutions);
    Ok(())
}

fn print_solutions<'a>(mut solns: Vec<Box<dyn AOCSolution + 'a>>) {
    for (day, sol) in solns.iter_mut().enumerate() {
        println!("Day {}:", day + 1); // Zero-indexed
        println!("Part 1: {}", sol.part_1());
        println!("Part 2: {}", sol.part_2());
        println!();
    }
}
