use std::fs::read_to_string;
use std::io::Result;

pub fn load_input(day: u32) -> Result<String> {
    read_to_string(format!("input/day{:0>2}/input.txt", day))
}
