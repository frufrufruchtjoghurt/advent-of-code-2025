use std::collections::{HashSet, VecDeque};

pub fn solve_part1(input: &str) -> usize {
    let mut beams: HashSet<usize> = HashSet::with_capacity(1);
    let mut split_count = 0;
    let mut add_beams: VecDeque<usize> = VecDeque::with_capacity(1024);
    let mut remove_beams: VecDeque<usize> = VecDeque::with_capacity(1024);

    let start_pos = input.find('S').unwrap();
    beams.insert(start_pos);

    input
        .lines()
        .enumerate()
        .filter(|(y, _)| y % 2 == 0)
        .skip(1)
        .for_each(|(_, line)| {
            for &beam in &beams {
                let char_at_beam = line.chars().nth(beam).unwrap();
                match char_at_beam {
                    '.' => {}
                    '^' => {
                        add_beams.push_back(beam - 1);
                        add_beams.push_back(beam + 1);
                        remove_beams.push_back(beam);
                        split_count += 1;
                    }
                    _ => {
                        remove_beams.push_back(beam);
                    }
                }
            }

            for beam in remove_beams.drain(..) {
                beams.remove(&beam);
            }
            for beam in add_beams.drain(..) {
                beams.insert(beam);
            }

            // _pretty_print(line, &beams);
        });

    split_count
}

pub fn solve_part2(input: &str) -> usize {
    let capacity = input.find('\n').unwrap();
    let mut beams: HashSet<usize> = HashSet::with_capacity(1);
    let mut path_count = vec![0; capacity];

    let start_pos = input.find('S').unwrap();
    path_count[start_pos] = 1;
    beams.insert(start_pos);

    input
        .lines()
        .enumerate()
        .filter(|(y, _)| y % 2 == 0)
        .skip(1)
        .for_each(|(_, line)| {
            beams = beams
                .drain()
                .filter_map(|beam| {
                    let char_at_beam = line.chars().nth(beam).unwrap();
                    match char_at_beam {
                        '.' => Some(vec![beam]),
                        '^' => {
                            path_count[beam - 1] += path_count[beam];
                            path_count[beam + 1] += path_count[beam];
                            path_count[beam] = 0;
                            Some(vec![beam - 1, beam + 1])
                        }
                        _ => None,
                    }
                })
                .flatten()
                .collect();

            // _pretty_print(line, &beams);
        });

    path_count.iter().sum()
}

fn _pretty_print(line: &str, beams: &HashSet<usize>) {
    for (i, ch) in line.chars().enumerate() {
        if beams.contains(&i) {
            print!("|");
        } else {
            print!("{}", ch);
        }
    }
    print!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn test_solve_part1() {
        let input = read_to_string("input/day07/example.txt").unwrap();
        let result = solve_part1(input.as_str());
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve_part2() {
        let input = read_to_string("input/day07/example.txt").unwrap();
        let result = solve_part2(input.as_str());
        assert_eq!(result, 40);
    }
}
