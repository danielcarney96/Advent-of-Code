use std::fs;

fn main() {
    let input = match fs::read_to_string("./input.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let largest_number = find_largest_possible_number_from_string(line, 2);

        sum += largest_number;
    }

    println!("{}", sum);
}

fn part2(input: &str) {
    let mut sum: i64 = 0;

    for line in input.lines() {
        let largest_number = find_largest_possible_number_from_string(line, 12);

        sum += largest_number;
    }

    println!("{}", sum);
}

fn find_largest_number_in_line_and_position(line: &str) -> (u32, usize) {
    const RADIX: u32 = 10;

    for number in (1..=9).rev() {
        for (line_char_index, line_char) in line.chars().enumerate() {
            if number == line_char.to_digit(RADIX).unwrap() {
                return (number, line_char_index);
            }
        }
    }

    return (0, 0);
}

fn find_largest_possible_number_from_string(line: &str, numbers_to_find: usize) -> i64 {
    let mut built_number = String::new();
    let mut absolute_position = 0;

    for i in (0..numbers_to_find).rev() {
        let window_start = absolute_position;
        let window_end = line.len().saturating_sub(i);

        if window_start >= window_end {
            break;
        }

        let trimmed_line = &line[window_start..window_end];

        let (found_number, found_number_index) =
            find_largest_number_in_line_and_position(trimmed_line);

        absolute_position = window_start + found_number_index + 1;
        built_number = format!("{}{}", built_number, found_number);
    }

    return built_number.parse::<i64>().unwrap();
}
