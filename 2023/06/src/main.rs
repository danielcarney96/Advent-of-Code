use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let (time_line, distance_line) = input.split_once("\n").unwrap();

    let times: Vec<i32> = get_numbers_from_line(time_line);
    let distances: Vec<i32> = get_numbers_from_line(distance_line);

    let mut ways_to_win_totals: Vec<i32> = Vec::new();

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let mut ways_to_win: i32 = 0;

        for t in 0..time {
            if t * (time - t) > distance {
                ways_to_win += 1;
            }
        }

        ways_to_win_totals.push(ways_to_win);
    }

    println!(
        "Part 1: {}",
        ways_to_win_totals.iter().fold(1, |acc, x| acc * x)
    );
}

fn part2(input: &String) {
    let (time_line, distance_line) = input.split_once("\n").unwrap();

    let time: i64 = get_numbers_from_line(time_line)
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, f| acc + &f)
        .parse::<i64>()
        .unwrap();
    let distance: i64 = get_numbers_from_line(distance_line)
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, f| acc + &f)
        .parse::<i64>()
        .unwrap();

    let mut ways_to_win: i32 = 0;

    for t in 0..time {
        if t * (time - t) > distance {
            ways_to_win += 1;
        }
    }

    println!("Part 2: {}", ways_to_win);
}

fn get_numbers_from_line(line: &str) -> Vec<i32> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
