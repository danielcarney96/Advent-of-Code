use std::fs;
use std::io;

fn main() {
    part1();
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

        let mut line_result = String::new();
        line_result.push(numbers[0]);
        line_result.push(numbers[numbers.len() - 1]);

        sum += line_result.parse::<i32>().unwrap();
    });

    println!("{}", sum);
}
