use std::process::exit;

use adventofcode2025::{CliArgs, run};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    if let Err(e) = run(args) {
        println!("An error occurred while running the program: {}", e);
        exit(1)
    }
}
