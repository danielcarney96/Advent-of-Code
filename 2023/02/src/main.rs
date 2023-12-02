use regex::Regex;
use std::{fs, io};

fn main() {
    part1();
}

fn part1() {
    let content: Result<String, io::Error> = fs::read_to_string("./input.txt");
    let colour_amount_pattern = Regex::new(r"(\d+)\s+(\w+)").unwrap();

    let mut sum: i32 = 0;
    let mut part2sum: i32 = 0;

    for (line_index, line) in content.unwrap().lines().enumerate() {
        let game_number = line_index + 1;

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

        part2sum += highest_red * highest_green * highest_blue;

        if highest_red < 13 && highest_green < 14 && highest_blue < 15 {
            sum += game_number as i32;
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", part2sum);
}
