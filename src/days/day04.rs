use std::error::Error;

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize, data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        if data.len() != width * height {
            return Err("Data length does not match grid dimensions".into());
        }
        Ok(Self {
            data,
            width,
            height,
        })
    }

    fn get(&self, x: usize, y: usize) -> &u8 {
        &self.data[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[x + y * self.width] = value;
    }
}

type Kernel3x3<T> = [[T; 3]; 3];
const NEIGHBOR_SUM: Kernel3x3<i8> = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];

fn calculate_removable_rolls(grid: &mut Grid, kernel: &Kernel3x3<i8>) -> usize {
    let mut removable_rolls = 0;
    let mut to_remove = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.get(x, y) == 0 {
                continue;
            }

            let mut sum: i8 = 0;
            for ky in 0..3 {
                for kx in 0..3 {
                    let offset_x = x as isize + kx as isize - 1;
                    let offset_y = y as isize + ky as isize - 1;

                    if offset_x < 0
                        || offset_y < 0
                        || offset_x >= grid.width as isize
                        || offset_y >= grid.height as isize
                    {
                        continue;
                    }

                    let value = grid.get(offset_x as usize, offset_y as usize);
                    sum += kernel[ky][kx] * (*value as i8);
                }
            }
            if sum < 4 {
                to_remove.push((x, y));
                removable_rolls += 1;
            }
        }
    }

    for (x, y) in to_remove {
        grid.set(x, y, 0);
    }

    removable_rolls
}

fn load_grid_from_str(input: &str) -> Result<Grid, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].chars().count();

    Grid::new(
        width,
        height,
        lines
            .iter()
            .flat_map(|line| line.chars().map(|c| if c == '@' { 1 } else { 0 }))
            .collect(),
    )
}

pub fn solve_part1(input: &str) -> usize {
    let mut grid = load_grid_from_str(input).unwrap();

    calculate_removable_rolls(&mut grid, &NEIGHBOR_SUM)
}

pub fn solve_part2(_input: &str) -> usize {
    let mut grid = load_grid_from_str(_input).unwrap();
    let mut total_removable = 0;

    loop {
        let removable = calculate_removable_rolls(&mut grid, &NEIGHBOR_SUM);

        if removable == 0 {
            break;
        }
        total_removable += removable;
    }
    total_removable
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs::read_to_string, vec};

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(3, 3, vec![0; 9]).unwrap();
        grid.set(1, 1, 5);
        assert_eq!(*grid.get(1, 1), 5);
    }

    #[test]
    fn test_load_grid_from_str() {
        let input = "@..\n..@\n.@.";
        let grid = load_grid_from_str(input).unwrap();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.data, vec![1, 0, 0, 0, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_solve_part1() {
        let input = read_to_string("input/day04/example.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve_part2() {
        let input = read_to_string("input/day04/example.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 43);
    }
}
