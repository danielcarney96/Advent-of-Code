use std::{collections::HashMap, fs, ops::Range};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let (first_line, rest) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = first_line.split("seeds: ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let group_strings = rest.split("\n\n").collect::<Vec<&str>>();

    let mut groups: HashMap<i64, Vec<(i64, i64, i64)>> = HashMap::new();

    for i in 0..group_strings.len() {
        let (_, group_lines) = group_strings[i].split_once("\n").unwrap();

        group_lines
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                let line_numbers: Vec<i64> = line
                    .split(" ")
                    .map(|val: &str| val.parse::<i64>().unwrap())
                    .collect();

                let new_value = (line_numbers[0], line_numbers[1], line_numbers[2]);

                if let Some(vector) = groups.get_mut(&(i as i64)) {
                    vector.push(new_value);
                } else {
                    let mut new_vector = Vec::new();
                    new_vector.push(new_value);
                    groups.insert(i as i64, new_vector);
                }
            });
    }

    let mut sources: Vec<i64> = Vec::new();

    for seed_index in 0..seeds.len() {
        let mut source: i64 = seeds[seed_index];

        for group_index in 0..groups.len() {
            source =
                find_destination_from_source(source, groups.get(&(group_index as i64)).unwrap());
        }

        sources.push(source);
    }

    println!("Part 1: {}", sources.iter().min().unwrap());
}

fn part2(input: &String) {
    let (first_line, rest) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = first_line.split("seeds: ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let group_strings = rest.split("\n\n").collect::<Vec<&str>>();

    let mut groups: HashMap<i64, Vec<(i64, i64, i64)>> = HashMap::new();

    for i in 0..group_strings.len() {
        let (_, group_lines) = group_strings[i].split_once("\n").unwrap();

        group_lines
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                let line_numbers: Vec<i64> = line
                    .split(" ")
                    .map(|val: &str| val.parse::<i64>().unwrap())
                    .collect();

                let new_value = (line_numbers[0], line_numbers[1], line_numbers[2]);

                if let Some(vector) = groups.get_mut(&(i as i64)) {
                    vector.push(new_value);
                } else {
                    let mut new_vector = Vec::new();
                    new_vector.push(new_value);
                    groups.insert(i as i64, new_vector);
                }
            });
    }

    let mut min_source: i64 = i64::MAX;

    for seed_index in (0..seeds.len()).step_by(2) {
        let mut ranges: Vec<Range<i64>> =
            vec![seeds[seed_index]..seeds[seed_index] + seeds[seed_index + 1]];

        for group_index in 0..groups.len() {
            let group = groups.get(&(group_index as i64)).unwrap();

            for group_line_index in 0..group.len() {
                ranges = find_destination_from_range(ranges, group[group_line_index]);
            }
        }

        min_source = min_source.min(ranges.iter().map(|range| range.start).min().unwrap());
    }

    println!("Part 2: {}", min_source);
}

fn find_destination_from_source(source: i64, group: &Vec<(i64, i64, i64)>) -> i64 {
    for i in 0..group.len() {
        if source >= group[i].1 && source < group[i].1 + group[i].2 {
            return group[i].0 + (source - group[i].1);
        }
    }

    return source;
}

fn find_destination_from_range(
    seed_ranges: Vec<Range<i64>>,
    group_line: (i64, i64, i64),
) -> Vec<Range<i64>> {
    let mut new_ranges = Vec::new();

    for i in 0..seed_ranges.len() {
        let group_line_range = group_line.1..group_line.1 + group_line.2;

        let intersection = seed_ranges[i].start.max(group_line_range.start)
            ..=seed_ranges[i].end.min(group_line_range.end);

        if intersection.is_empty() {
            new_ranges.push(seed_ranges[i].clone());
        } else {
            if *intersection.start() > seed_ranges[i].start {
                new_ranges.push(seed_ranges[i].start..*intersection.start());
            }
            if *intersection.end() < seed_ranges[i].end {
                new_ranges.push(seed_ranges[i].end..*intersection.end());
            }

            new_ranges.push(
                group_line.0 + intersection.start() - group_line.1
                    ..group_line.0 + intersection.end() - group_line.1,
            );
        }
    }

    return new_ranges;
}
