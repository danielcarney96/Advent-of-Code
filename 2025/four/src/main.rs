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
    Ok(())
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

    for row_offset in -1i32..=1i32 {
        for col_offset in -1i32..=1i32 {
            // Skip the reference position itself
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let neighbor_row = ref_row as i32 + row_offset;
            let neighbor_col = ref_col as i32 + col_offset;

            // Check bounds
            if neighbor_row < 0
                || neighbor_col < 0
                || neighbor_row >= grid.len() as i32
                || neighbor_col >= grid[0].len() as i32
            {
                continue;
            }

            let neighbor = grid[neighbor_row as usize][neighbor_col as usize];
            if is_roll(neighbor) {
                neighbours_found += 1;
            }
        }
    }

    return Some(neighbours_found);
}
