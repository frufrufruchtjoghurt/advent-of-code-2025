pub fn solve_part1(input: &str) -> usize {
    let line_count = input.lines().count();
    let ops: Vec<&str> = input.lines().last().unwrap().split_whitespace().collect();
    let mut results = vec![0; ops.len()];

    for (i, line) in input.lines().enumerate() {
        if i == line_count - 1 {
            break;
        }
        for (j, num_str) in line.split_whitespace().enumerate() {
            match ops[j] {
                "+" => {
                    results[j] += num_str.parse::<usize>().unwrap();
                }
                "*" => {
                    if i == 0 {
                        results[j] = num_str.parse::<usize>().unwrap();
                    } else {
                        results[j] *= num_str.parse::<usize>().unwrap();
                    }
                }
                _ => unreachable!("Only + and * operations are supported"),
            }
        }
    }

    results.iter().sum()
}

pub fn solve_part2(input: &str) -> usize {
    let num_line_count = input.lines().count() - 1;
    let ops: Vec<(usize, char)> = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .collect();
    let mut results = vec![0; ops.len()];

    let num_char_grid: Vec<Vec<char>> = input
        .lines()
        .take(num_line_count)
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..ops.len() {
        // the block starts at the position of the ith character that is not a whitespace in ops
        let block_start = ops[i].0;
        let block_end = if i + 1 < ops.len() {
            ops[i + 1].0 - 2
        } else {
            num_char_grid[0].len() - 1
        };
        let operation = ops[i].1;

        for col in (block_start..=block_end).rev() {
            let mut num_str = String::with_capacity(num_line_count);

            for row in (0..num_line_count).rev() {
                let c = num_char_grid[row][col];
                if !c.is_ascii_digit() && num_str.contains(|c: char| c.is_ascii_digit()) {
                    break;
                }
                num_str.push(c);
            }

            let num = num_str
                .trim()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            match operation {
                '+' => {
                    results[i] += num;
                }
                '*' => {
                    if col == block_end {
                        results[i] = num;
                    } else {
                        results[i] *= num;
                    }
                }
                _ => unreachable!("Only + and * operations are supported"),
            }
        }
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = read_to_string("input/day06/example.txt").unwrap();
        let result = solve_part1(input.as_str());
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_part2() {
        let input = read_to_string("input/day06/example.txt").unwrap();
        let result = solve_part2(input.as_str());
        assert_eq!(result, 3263827);
    }
}
