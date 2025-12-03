pub fn solve_part1(input: &str) -> u64 {
    let battery_stacks = load_battery_stacks(input);

    battery_stacks
        .iter()
        .map(|batteries| max_battery_joltage(batteries, 2))
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    let battery_stacks = load_battery_stacks(input);

    battery_stacks
        .iter()
        .map(|batteries| max_battery_joltage(batteries, 12))
        .sum()
}

/// Calculates the maximum possible joltage from a stack of batteries using a monotonic decreasing stack.
///
/// # Arguments
/// * `batteries` - A vector of battery joltage ratings (1-9)
/// * `keep` - The amount of batteries that should be combined for maximum joltage
///
fn max_battery_joltage(batteries: &Vec<u8>, keep: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::new();
    let batteries_len = batteries.len();

    for (i, &battery) in batteries.iter().enumerate() {
        while !stack.is_empty()
            && *stack.last().unwrap() < battery
            && stack.len() + (batteries_len - i) > keep
        {
            stack.pop();
        }

        if stack.len() < keep {
            stack.push(battery);
        }
    }
    stack
        .iter()
        .fold(0 as u64, |acc, &digit| acc * 10 + digit as u64)
}

fn load_battery_stacks(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_battery_stacks() {
        let input = "123\n456\n789";
        let stacks = load_battery_stacks(input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec![1, 2, 3]);
        assert_eq!(stacks[1], vec![4, 5, 6]);
        assert_eq!(stacks[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_example_part1() {
        let input = read_to_string("input/day03/example.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_example_part2() {
        let input = read_to_string("input/day03/example.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 3121910778619);
    }
}
