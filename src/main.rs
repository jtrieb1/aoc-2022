#[macro_use]
mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

pub use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions: Vec<Box<dyn AOCSolution>> = vec![
        day1::solution()?,
        day2::solution()?,
        day3::solution()?,
        day4::solution()?,
        day5::solution()?,
        day6::solution()?,
        day7::solution()?,
        day8::solution()?,
        day9::solution()?,
        day10::solution()?,
        day11::solution()?,
        day12::solution()?,
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
