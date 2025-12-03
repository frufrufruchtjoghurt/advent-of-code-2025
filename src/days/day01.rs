const DIAL_SIZE: i32 = 100;

struct Dial(i32);

impl Dial {
    fn new() -> Self {
        Dial(50)
    }

    /// Moves the dial by the specified amount of steps.
    /// Positive values move the dial to the right, negative to the left.
    ///
    /// # Constraints
    /// The dial wraps around at 0 and 99.
    ///
    /// # Returns
    /// The number of times the dial passed the zero position during the move.
    /// ATTENTION: Landing on zero does NOT count as passing zero.
    fn move_dial(&mut self, steps: i32) -> u32 {
        let full_roations = steps.abs() / DIAL_SIZE;

        let steps = steps % DIAL_SIZE;

        let mut passed_zero = full_roations as u32;

        if !self.is_zero() && (self.0 + steps > DIAL_SIZE || self.0 + steps < 0) {
            passed_zero += 1;
        }

        self.0 = (self.0 + steps).rem_euclid(DIAL_SIZE);

        passed_zero
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

fn load_dial_changes(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let value = line[1..].parse::<u32>().unwrap();
            match direction {
                'R' => value as i32,
                'L' => -(value as i32),
                _ => unreachable!("Invalid direction character"),
            }
        })
        .collect()
}

pub fn solve_part1(input: &str) -> u32 {
    let instructions = load_dial_changes(input);
    let mut dial = Dial::new();

    let zero_count = instructions.iter().fold(0, |acc: u32, instr| {
        dial.move_dial(*instr);
        acc + if dial.is_zero() { 1 } else { 0 }
    });

    zero_count
}

pub fn solve_part2(input: &str) -> u32 {
    let instructions = load_dial_changes(input);
    let mut dial = Dial::new();

    let (zero_count, passed_zero_count) =
        instructions.iter().fold((0, 0), |acc: (u32, u32), instr| {
            let passed = dial.move_dial(*instr);
            let zero_hits = if dial.is_zero() { 1 } else { 0 };
            (acc.0 + zero_hits, acc.1 + passed)
        });

    passed_zero_count + zero_count
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_file() {
        let input = read_to_string("input/day1/test_input.txt").unwrap();
        let instructions = load_dial_changes(input.as_str());
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0], 10);
        assert_eq!(instructions[1], -20);
        assert_eq!(instructions[2], 30);
        assert_eq!(instructions[3], -40);
    }

    #[test]
    fn test_example() {
        let input = read_to_string("input/day1/example.txt").unwrap();
        let instructions = load_dial_changes(input.as_str());
        let mut dial = Dial::new();

        let (zero_count, passed_zero_count) =
            instructions.iter().fold((0, 0), |acc: (u32, u32), instr| {
                let passed = dial.move_dial(*instr);
                let zero_hits = if dial.is_zero() { 1 } else { 0 };
                (acc.0 + zero_hits, acc.1 + passed)
            });

        assert_eq!(dial.0, 32);
        assert_eq!(zero_count, 3);
        assert_eq!(passed_zero_count, 3);
        assert_eq!(zero_count + passed_zero_count, 6);
    }
}
