use std::fs;
use std::io;

fn main() {
    part1();
    part2();
}

fn part1() {
    let content: Result<String, io::Error> = fs::read_to_string("./input.txt");

    let mut sum: i32 = 0;

    content.unwrap().lines().for_each(|line: &str| {
        let mut numbers: Vec<char> = Vec::new();

        line.chars().for_each(|c| {
            if c.is_numeric() {
                numbers.push(c)
            }
        });

        if numbers.len() == 0 {
            return;
        }

        let mut line_result = String::new();
        line_result.push(numbers[0]);
        line_result.push(numbers[numbers.len() - 1]);

        sum += line_result.parse::<i32>().unwrap();
    });

    println!("Part 1: {}", sum);
}

fn part2() {
    let content: Result<String, io::Error> = fs::read_to_string("./input.txt");

    let mut sum: i32 = 0;
    const NUMBERS_AS_STRING: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    content.unwrap().lines().for_each(|line: &str| {
        let mut first_number: Option<char> = None;
        let mut first_number_index: Option<usize> = None;

        let mut last_number: Option<char> = None;
        let mut last_number_index: Option<usize> = None;

        for (index, num_as_string) in NUMBERS_AS_STRING.iter().enumerate() {
            let numerical_found_at: Vec<_> =
                line.match_indices(index.to_string().as_str()).collect();
            let string_found_at: Vec<_> = line.match_indices(num_as_string).collect();

            // Nothing found on this line
            if numerical_found_at.len() < 1 && string_found_at.len() < 1 {
                continue;
            }

            let char_value = char::from_digit(index as u32, 10).unwrap();

            let mut update_numbers = |found_at: Option<usize>| {
                if let Some(found_index) = found_at {
                    if found_index < first_number_index.unwrap_or(usize::MAX) {
                        first_number = Some(char_value);
                        first_number_index = found_at;
                    }

                    if found_index > last_number_index.unwrap_or(usize::MIN) {
                        last_number = Some(char_value);
                        last_number_index = found_at;
                    }
                }
            };

            numerical_found_at.iter().for_each(|(found_at, _)| {
                update_numbers(Some(*found_at));
            });
            string_found_at.iter().for_each(|(found_at, _)| {
                update_numbers(Some(*found_at));
            });
        }

        // To cover instances where only one number exists in the line
        if first_number == None {
            first_number = last_number;
        }
        if last_number == None {
            last_number = first_number;
        }

        sum += format!("{}{}", first_number.unwrap(), last_number.unwrap())
            .parse::<i32>()
            .unwrap();
    });

    println!("Part 2: {}", sum);
}
