use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    part1(grid.clone());
    part2(grid.clone());
}

fn part1(mut grid: Vec<Vec<char>>) {
    let mut sum: i32 = 0;

    let symbol_positions_in_grid: Vec<(usize, usize)> = find_symbol_positions(&grid);

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            // Only look at numbers
            if !grid[y][x].is_alphanumeric() {
                continue;
            }

            let found_adjacent_symbol = symbol_positions_in_grid
                .iter()
                .any(|(symbol_x, symbol_y)| is_adjacent(x, y, *symbol_x, *symbol_y));

            // Number isn't adjacent to a symbol so continue
            if !found_adjacent_symbol {
                continue;
            }

            let mut connected_numbers: Vec<char> = Vec::new();

            connected_numbers.push(grid[y][x]);
            grid[y][x] = '.';

            // Check left of connected number
            for i in (0..x).rev() {
                if !grid[y][i].is_alphanumeric() {
                    break;
                }

                // To the left so add to start
                connected_numbers.insert(0, grid[y][i]);
                grid[y][i] = '.';
            }

            // Check right of connected number
            for i in (x + 1)..grid.len() {
                if !grid[y][i].is_alphanumeric() {
                    break;
                }

                // To the right so add to end
                connected_numbers.push(grid[y][i]);
                grid[y][i] = '.';
            }

            // Merge the array into a string then convert to i32 and add to sum
            sum += connected_numbers
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(mut grid: Vec<Vec<char>>) {
    let mut gear_positions: Vec<(usize, usize)> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if item == &'*' {
                gear_positions.push((x, y));
            }
        }
    }

    let mut gear_numbers: HashMap<&(usize, usize), Vec<i32>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            // Only look at numbers
            if !grid[y][x].is_alphanumeric() {
                continue;
            }

            let adjacent_at_gear_position = gear_positions
                .iter()
                .find(|(symbol_x, symbol_y)| is_adjacent(x, y, *symbol_x, *symbol_y));

            // Number isn't adjacent to a gear so continue
            if adjacent_at_gear_position.is_none() {
                continue;
            }

            let mut connected_numbers: Vec<char> = Vec::new();

            connected_numbers.push(grid[y][x]);
            grid[y][x] = '.';

            // Check left of connected number
            for i in (0..x).rev() {
                if !grid[y][i].is_alphanumeric() {
                    break;
                }

                // To the left so add to start
                connected_numbers.insert(0, grid[y][i]);
                grid[y][i] = '.';
            }

            // Check right of connected number
            for i in (x + 1)..grid.len() {
                if !grid[y][i].is_alphanumeric() {
                    break;
                }

                // To the right so add to end
                connected_numbers.push(grid[y][i]);
                grid[y][i] = '.';
            }

            let entry = gear_numbers
                .entry(adjacent_at_gear_position.unwrap())
                .or_insert(Vec::new());

            entry.push(
                connected_numbers
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            );
        }
    }

    println!(
        "Part 2: {}",
        gear_numbers.values().fold(0, |acc, row| {
            if row.len() < 2 {
                return acc;
            }

            let multiplied_row = row.iter().fold(1, |acc, item| acc * item);
            return acc + multiplied_row;
        })
    );
}

fn find_symbol_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if is_symbol(item) {
                positions.push((x, y));
            }
        }
    }

    return positions;
}

fn is_symbol(symbol: &char) -> bool {
    return !symbol.is_alphanumeric() && symbol != &'.';
}

fn is_adjacent(row1: usize, col1: usize, row2: usize, col2: usize) -> bool {
    let row_diff = (row1 as i32 - row2 as i32).abs();
    let col_diff = (col1 as i32 - col2 as i32).abs();

    return row_diff <= 1 && col_diff <= 1;
}
