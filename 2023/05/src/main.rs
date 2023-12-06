use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1(&input);
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

fn find_destination_from_source(source: i64, group: &Vec<(i64, i64, i64)>) -> i64 {
    for i in 0..group.len() {
        if source >= group[i].1 && source < group[i].1 + group[i].2 {
            return group[i].0 + (source - group[i].1);
        }
    }

    return source;
}
