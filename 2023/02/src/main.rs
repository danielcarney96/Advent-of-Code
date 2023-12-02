use regex::Regex;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut sum: i32 = 0;

    for (line_index, line) in input.lines().enumerate() {
        let (highest_red, highest_green, highest_blue) = find_highest_for_each_colour_in_line(line);

        if highest_red < 13 && highest_green < 14 && highest_blue < 15 {
            sum += (line_index + 1) as i32;
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &String) {
    let mut sum: i32 = 0;

    for (_, line) in input.lines().enumerate() {
        let (highest_red, highest_green, highest_blue) = find_highest_for_each_colour_in_line(line);

        sum += highest_red * highest_green * highest_blue;
    }

    println!("Part 2: {}", sum);
}

fn find_highest_for_each_colour_in_line(line: &str) -> (i32, i32, i32) {
    let colour_amount_pattern = Regex::new(r"(\d+)\s+(\w+)").unwrap();

    let mut highest_red: i32 = 0;
    let mut highest_green: i32 = 0;
    let mut highest_blue: i32 = 0;

    for captures in colour_amount_pattern.captures_iter(line) {
        let amount = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let color = captures.get(2).unwrap().as_str();

        match color {
            "red" => {
                if amount > highest_red {
                    highest_red = amount;
                }
            }
            "green" => {
                if amount > highest_green {
                    highest_green = amount;
                }
            }
            "blue" => {
                if amount > highest_blue {
                    highest_blue = amount;
                }
            }
            _ => {}
        }
    }

    return (highest_red, highest_green, highest_blue);
}
