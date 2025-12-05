pub fn solve_part1(input: &str) -> usize {
    let (ranges, ids) = load_ranges_and_ids(input);
    ids.into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(*id)))
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    let (ranges, _ids) = load_ranges_and_ids(input);

    // Combine overlapping ranges
    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|range| range.start);

    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    fn contains(&self, value: usize) -> bool {
        value >= self.start && value <= self.end
    }
}

fn load_ranges_and_ids(input: &str) -> (Vec<Range>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut is_ids = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            is_ids = true;
            continue;
        }

        if !is_ids {
            let parts: Vec<usize> = line
                .split('-')
                .map(|num| num.parse::<usize>().expect("Failed to parse number"))
                .collect::<Vec<usize>>();
            let start = parts[0];
            let end = parts[1];
            ranges.push(Range::new(start, end));
        } else {
            let id = line.trim().parse::<usize>().expect("Failed to parse ID");
            ids.push(id);
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn test_load_ranges_and_ids() {
        let input = read_to_string("input/day05/example.txt").unwrap();
        let (ranges, ids) = load_ranges_and_ids(&input);

        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0].start, 3);
        assert_eq!(ranges[0].end, 5);
        assert_eq!(ranges[1].start, 10);
        assert_eq!(ranges[1].end, 14);
        assert_eq!(ranges[2].start, 16);
        assert_eq!(ranges[2].end, 20);
        assert_eq!(ranges[3].start, 12);
        assert_eq!(ranges[3].end, 18);
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_solve_part1() {
        let input = read_to_string("input/day05/example.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part2() {
        let input = read_to_string("input/day05/example.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 14);
    }
}
