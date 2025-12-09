mod utils;
use std::error::Error;

pub use utils::*;

mod days;
pub use days::*;

use clap::{Parser, command};

// A simple command-line application for the Advent of Code 2025
#[derive(Parser)]
#[command(name = "Advent of Code 2025")]
#[command(about = "Run solutions for Advent of Code 2025", long_about = None)]
#[command(author = "Markus Fruhmann")]
pub struct CliArgs {
    // The day that should be executed (1-12)
    #[arg(short, long)]
    day: u32,
}

pub fn run(args: CliArgs) -> Result<(), Box<dyn Error>> {
    let input = load_input(args.day)?;

    println!("The solutions for day {} are:", args.day);
    match args.day {
        1 => println!(
            "Part 1: {}\nPart 2: {}",
            day01::solve_part1(input.as_str()),
            day01::solve_part2(input.as_str())
        ),
        2 => println!(
            "Part 1: {}\nPart 2: {}",
            day02::solve_part1(input.as_str()),
            day02::solve_part2(input.as_str())
        ),
        3 => println!(
            "Part 1: {}\nPart 2: {}",
            day03::solve_part1(input.as_str()),
            day03::solve_part2(input.as_str())
        ),
        4 => println!(
            "Part 1: {}\nPart 2: {}",
            day04::solve_part1(input.as_str()),
            day04::solve_part2(input.as_str())
        ),
        5 => println!(
            "Part 1: {}\nPart 2: {}",
            day05::solve_part1(input.as_str()),
            day05::solve_part2(input.as_str())
        ),
        6 => println!(
            "Part 1: {}\nPart 2: {}",
            day06::solve_part1(input.as_str()),
            day06::solve_part2(input.as_str())
        ),
        7 => println!(
            "Part 1: {}\nPart 2: {}",
            day07::solve_part1(input.as_str()),
            day07::solve_part2(input.as_str())
        ),
        8 => println!(
            "Part 1: {}\nPart 2: {}",
            day08::solve_part1(input.as_str()),
            day08::solve_part2(input.as_str())
        ),
        _ => println!("Solution for Day {} is not yet implemented.", args.day),
    }
    Ok(())
}
