use adventofcode2025::{CliArgs, run};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    run(args);
}
