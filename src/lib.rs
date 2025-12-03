mod days;
use days::*;

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

pub fn run(args: CliArgs) {
    match args.day {
        1 => day01::solve(),
        2 => day02::solve(),
        _ => println!("Solution for Day {} is not yet implemented.", args.day),
    }
}
