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
        // check line minus last char (as you cannot then get a second char if first comes from the end)
        // largest total number should always start with the biggest digit
        let trimmed_line = &line[..line.len().saturating_sub(1)];
        let (first_number, first_number_index) =
            find_largest_number_in_line_and_position(trimmed_line);

        // check remaining line after index of first char
        let (_, remaining_line) = line.split_at(first_number_index + 1);
        let (second_number, _) = find_largest_number_in_line_and_position(remaining_line);

        let result = format!("{}{}", first_number, second_number)
            .parse::<i32>()
            .unwrap();

        sum += result;
    }

    println!("{}", sum);
}

fn part2(input: &str) {
    let mut sum: i64 = 0;

    const NUMBERS_TO_FIND: usize = 12;

    for line in input.lines() {
        let mut built_number = String::new();
        let mut absolute_position = 0;

        for i in (0..NUMBERS_TO_FIND).rev() {
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

        sum += built_number.parse::<i64>().unwrap();
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
