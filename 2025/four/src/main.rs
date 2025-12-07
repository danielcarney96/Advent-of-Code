use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    part1(&grid)?;
    part2(&grid)?;

    Ok(())
}

fn part1(grid: &Vec<Vec<char>>) -> Result<()> {
    let mut accessibile_papers: i32 = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let neighbour_count: Option<i32> =
                get_count_of_neighbouring_rolls((x, y), |item| item == '@', grid);

            match neighbour_count {
                Some(result) => {
                    if result < 4 {
                        accessibile_papers += 1;
                    }
                }
                None => continue,
            }
        }
    }

    println!("{}", accessibile_papers);

    Ok(())
}
fn part2(grid: &Vec<Vec<char>>) -> Result<()> {
    let mut rolls_removed: i32 = 0;
    let mut grid_copy = grid.clone();

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let newly_removed_rolls: Option<i32> =
                remove_roll_and_check_recheck_neighbours((x, y), &mut grid_copy);

            match newly_removed_rolls {
                Some(result) => rolls_removed += result,
                None => continue,
            }
        }
    }

    println!("{}", rolls_removed);

    Ok(())
}

fn remove_roll_and_check_recheck_neighbours(
    reference_position: (usize, usize),
    grid: &mut Vec<Vec<char>>,
) -> Option<i32> {
    let mut rolls_removed = 0;
    let is_roll = |item| item == '@';

    let (x, y) = (reference_position.0, reference_position.1);

    let neighbour_count: Option<i32> = get_count_of_neighbouring_rolls((x, y), is_roll, grid);

    match neighbour_count {
        Some(result) => {
            if result < 4 {
                grid[reference_position.0][reference_position.1] = '.';
                rolls_removed += 1;

                let neighbor_positions = get_neighbor_positions(reference_position, grid);

                for (row, col) in neighbor_positions {
                    let neighbor = grid[row][col];
                    if is_roll(neighbor) {
                        let newly_removed_rolls: Option<i32> =
                            remove_roll_and_check_recheck_neighbours((row, col), grid);

                        match newly_removed_rolls {
                            Some(result) => rolls_removed += result,
                            None => continue,
                        }
                    }
                }
            }
        }
        None => return None,
    }

    return Some(rolls_removed);
}

fn get_count_of_neighbouring_rolls(
    reference_position: (usize, usize),
    is_roll: fn(item: char) -> bool,
    grid: &Vec<Vec<char>>,
) -> Option<i32> {
    let (ref_row, ref_col) = reference_position;
    let item = grid[ref_row][ref_col];

    if !is_roll(item) {
        return None;
    }

    let mut neighbours_found: i32 = 0;
    let neighbor_positions = get_neighbor_positions(reference_position, grid);

    for (row, col) in neighbor_positions {
        let neighbor = grid[row][col];
        if is_roll(neighbor) {
            neighbours_found += 1;
        }
    }

    return Some(neighbours_found);
}

fn get_neighbor_positions(
    reference_position: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let mut neighbor_positions = Vec::new();
    let (ref_row, ref_col) = reference_position;

    for row_offset in -1i32..=1i32 {
        for col_offset in -1i32..=1i32 {
            // Skip the reference position itself
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let neighbor_row = ref_row as i32 + row_offset;
            let neighbor_col = ref_col as i32 + col_offset;

            // Check bounds
            if neighbor_row >= 0
                && neighbor_col >= 0
                && neighbor_row < grid.len() as i32
                && neighbor_col < grid[0].len() as i32
            {
                neighbor_positions.push((neighbor_row as usize, neighbor_col as usize));
            }
        }
    }

    return neighbor_positions;
}
