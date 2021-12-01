fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let lines: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let count: usize = lines.windows(2)
        .filter(|window| window[0] < window[1])
        .count();

    println!("{}", count);
}

pub fn part_two() {
    let lines: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let grouped_sum: Vec<u32> = lines.windows(3)
        .map(|i| i.iter().sum())
        .collect();

    let count: usize = grouped_sum.windows(2)
        .filter(|window| window[0] < window[1])
        .count();

    println!("{}", count);
}
