use std::ops::{Add, AddAssign, Sub, SubAssign};

// Valid dial positions range from 0 to 99
#[derive(Clone, Copy)]
struct Dial {
    position: u32,
    zero_count: u32,
    passed_zero_count: u32,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            zero_count: 0,
            passed_zero_count: 0,
        }
    }
}

impl Add<u32> for Dial {
    type Output = Dial;

    fn add(self, rhs: u32) -> Dial {
        let new_position = self.position + rhs;

        let passes = new_position / 100;
        let new_position = new_position % 100;

        if new_position == 0 {
            Dial {
                position: new_position,
                zero_count: self.zero_count + 1,
                passed_zero_count: self.passed_zero_count + passes,
            }
        } else {
            Dial {
                position: new_position,
                zero_count: self.zero_count,
                passed_zero_count: self.passed_zero_count + passes,
            }
        }
    }
}

impl Sub<u32> for Dial {
    type Output = Dial;

    fn sub(self, rhs: u32) -> Dial {
        let rhs_remaining = rhs % 100;
        let mut passes = rhs / 100;

        let new_position = if self.position < rhs_remaining {
            if self.position != 0 {
                passes += 1;
            }
            100 + self.position - rhs_remaining
        } else {
            self.position - rhs_remaining
        };

        if new_position == 0 {
            Dial {
                position: new_position,
                zero_count: self.zero_count + 1,
                passed_zero_count: self.passed_zero_count + passes + 1,
            }
        } else {
            Dial {
                position: new_position,
                zero_count: self.zero_count,
                passed_zero_count: self.passed_zero_count + passes,
            }
        }
    }
}

impl AddAssign<u32> for Dial {
    fn add_assign(&mut self, rhs: u32) {
        let result = *self + rhs;
        self.position = result.position;
        self.zero_count = result.zero_count;
        self.passed_zero_count = result.passed_zero_count;
    }
}

impl SubAssign<u32> for Dial {
    fn sub_assign(&mut self, rhs: u32) {
        let result = *self - rhs;
        self.position = result.position;
        self.zero_count = result.zero_count;
        self.passed_zero_count = result.passed_zero_count;
    }
}

fn load_file(path: &str) -> Vec<(char, u32)> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    content
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let value = line[1..].parse::<u32>().unwrap();
            (direction, value)
        })
        .collect()
}

fn main() {
    let instructions = load_file("input/input.txt");
    let mut dial = Dial::new();

    for (direction, value) in instructions {
        match direction {
            'R' => dial += value,
            'L' => dial -= value,
            _ => panic!("Invalid direction"),
        }
    }

    println!("Final Dial Position: {}", dial.position);
    println!("Number of times dial hit zero: {}", dial.zero_count);
    println!(
        "Number of times dial passed zero: {}",
        dial.passed_zero_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        let instructions = load_file("input/test_input.txt");
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0], ('R', 10));
        assert_eq!(instructions[1], ('L', 20));
        assert_eq!(instructions[2], ('R', 30));
        assert_eq!(instructions[3], ('L', 40));
    }

    #[test]
    fn test_example() {
        let instructions = load_file("input/example.txt");
        let mut dial = Dial::new();
        for (direction, value) in instructions {
            match direction {
                'R' => dial += value,
                'L' => dial -= value,
                _ => panic!("Invalid direction"),
            }
        }
        assert_eq!(dial.position, 32);
        assert_eq!(dial.zero_count, 3);
        assert_eq!(dial.passed_zero_count, 6);
    }
}
