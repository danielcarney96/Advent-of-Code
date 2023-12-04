use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut sum: i32 = 0;

    for (_, line) in input.lines().enumerate() {
        let split = line.split(": ").collect::<Vec<&str>>();
        let numbers_split = split[1].split(" | ").collect::<Vec<&str>>();

        let winning_numbers = string_to_number_array(numbers_split[0]);
        let numbers_owned = string_to_number_array(numbers_split[1]);

        sum += numbers_owned.iter().fold(0, |acc, row| {
            if winning_numbers.contains(&row) {
                if acc == 0 {
                    return 1;
                }

                return acc * 2;
            }

            return acc;
        });
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &String) {
    let mut wins_for_each_line: HashMap<i32, (i32, i32)> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let split = line.split(": ").collect::<Vec<&str>>();
        let numbers_split = split[1].split(" | ").collect::<Vec<&str>>();

        let winning_numbers = string_to_number_array(numbers_split[0]);
        let numbers_owned = string_to_number_array(numbers_split[1]);

        wins_for_each_line.insert(
            i as i32,
            (
                numbers_owned.iter().fold(0, |acc, row| {
                    if winning_numbers.contains(&row) {
                        return acc + 1;
                    }

                    return acc;
                }),
                1,
            ),
        );
    }

    let mut sum: i32 = 0;

    for i in 0..wins_for_each_line.len() {
        let line = wins_for_each_line[&(i as i32)];

        sum += line.1;

        for j in 0..line.0 {
            let line_to_update = wins_for_each_line[&((i + 1) as i32 + j)];

            *wins_for_each_line
                .entry(((i + 1) as i32 + j) as i32)
                .or_insert((line_to_update.0, line_to_update.1)) =
                (line_to_update.0, line_to_update.1 + 1 * line.1);
        }
    }

    println!("Part 2: {}", sum);
}

fn string_to_number_array(input: &str) -> Vec<i32> {
    input
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
