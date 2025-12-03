use itertools::Itertools;

fn load_ranges(filename: &str) -> Vec<(u128, u128)> {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    content
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|num| num.parse::<u128>().expect("Failed to parse number"))
                .collect_tuple()
                .expect("Failed to collect tuple")
        })
        .collect()
}

fn is_valid_id_part1(id: u128) -> bool {
    let id_len = id.to_string().len();
    if id_len < 2 || id_len % 2 != 0 {
        return true;
    }

    let id_str = id.to_string();
    let half_len = id_len / 2;

    let left_half = &id_str[..half_len].parse::<u128>().unwrap();
    let right_half = &id_str[half_len..].parse::<u128>().unwrap();

    left_half != right_half
}

fn is_valid_id_part2(id: u128) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();
    if id_len < 2 {
        return true;
    }

    for pat_len in 1..=(id_len / 2) {
        if id_len % pat_len != 0 {
            continue;
        }
        let pattern = &id_str[..pat_len];
        if pattern.repeat(id_len / pat_len) == id_str {
            return false;
        }
    }
    true
}

fn sum_invalid_ids_in_ranges(ranges: &Vec<(u128, u128)>, is_valid_id: fn(u128) -> bool) -> u128 {
    let mut sum = 0;
    for (start, end) in ranges {
        for id in *start..=*end {
            if !is_valid_id(id) {
                sum += id;
            }
        }
    }
    sum
}

pub fn solve() {
    let ranges = load_ranges("input/day2/input.txt");
    let sum = sum_invalid_ids_in_ranges(&ranges, is_valid_id_part1);
    println!("Sum of invalid IDs according to part 1: {}", sum);
    let sum = sum_invalid_ids_in_ranges(&ranges, is_valid_id_part2);
    println!("Sum of invalid IDs according to part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ranges() {
        let ranges = load_ranges("input/day2/example.txt");
        assert_eq!(ranges.len(), 11);
    }

    #[test]
    fn test_is_valid_id_part1() {
        assert_eq!(is_valid_id_part1(11), false);
        assert_eq!(is_valid_id_part1(12), true);
        assert_eq!(is_valid_id_part1(1234), true);
        assert_eq!(is_valid_id_part1(1212), false);
        assert_eq!(is_valid_id_part1(123456), true);
        assert_eq!(is_valid_id_part1(123123), false);
        assert_eq!(is_valid_id_part1(1), true);
    }

    #[test]
    fn test_is_valid_id_part2() {
        assert_eq!(is_valid_id_part2(11), false);
        assert_eq!(is_valid_id_part2(12), true);
        assert_eq!(is_valid_id_part2(1234), true);
        assert_eq!(is_valid_id_part2(1212), false);
        assert_eq!(is_valid_id_part2(123456), true);
        assert_eq!(is_valid_id_part2(123123), false);
        assert_eq!(is_valid_id_part2(1), true);
    }

    #[test]
    fn test_example() {
        let ranges = load_ranges("input/day2/example.txt");
        let sum = sum_invalid_ids_in_ranges(&ranges, is_valid_id_part1);
        assert_eq!(sum, 1227775554); // result for part 1
        let sum = sum_invalid_ids_in_ranges(&ranges, is_valid_id_part2);
        assert_eq!(sum, 4174379265); // result for part 2
    }
}
