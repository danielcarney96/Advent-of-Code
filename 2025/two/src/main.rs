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

    input.split(",").for_each(|group| {
        let parts: Vec<&str> = group.split("-").collect();

        if parts.len() != 2 {
            return;
        }

        let first: i64 = parts[0].parse().unwrap();
        let second: i64 = parts[1].parse().unwrap();

        for i in first..=second {
            let i_as_str = i.to_string();

            // cannot be odd and repeat
            if i_as_str.len() % 2 != 0 {
                continue;
            }

            let (first_half, second_half): (&str, &str) = i_as_str.split_at(i_as_str.len() / 2);

            if first_half == second_half {
                sum += i;
            }
        }
        return;
    });

    println!("{}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;

    input.split(",").for_each(|group| {
        let parts: Vec<&str> = group.split("-").collect();

        if parts.len() != 2 {
            return;
        }

        let first: i64 = parts[0].parse().unwrap();
        let second: i64 = parts[1].parse().unwrap();

        for i in first..second + 1 {
            let i_as_str = i.to_string();

            let min_group_size = 1;
            let max_group_size: i32 = (i_as_str.len() / 2).try_into().unwrap();

            for group_size in min_group_size..=max_group_size {
                // Only check group sizes that can evenly divide the string length
                if i_as_str.len() % (group_size as usize) != 0 {
                    continue;
                }

                let group_parts: Vec<String> = i_as_str
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(group_size as usize)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect();

                if group_parts.len() > 1 && group_parts.iter().all(|part| part == &group_parts[0]) {
                    sum += i;
                    break;
                }
            }
        }
        return;
    });

    println!("{}", sum);
}
