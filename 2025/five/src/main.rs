use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    let (id_ranges_str, ids_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow::anyhow!("Bad parse :("))?;

    let id_ranges: Vec<(i64, i64)> = id_ranges_str
        .lines()
        .map(|line| -> Result<(i64, i64)> {
            let (item1, item2) = line
                .split_once("-")
                .ok_or_else(|| anyhow::anyhow!("Line missing '-' separator: {}", line))?;
            Ok((parse_to_int(item1)?, parse_to_int(item2)?))
        })
        .collect::<Result<Vec<_>>>()?;

    let ids: Vec<i64> = ids_str
        .lines()
        .map(|line| parse_to_int(line))
        .collect::<Result<Vec<_>>>()?;

    part1(&id_ranges, &ids)?;
    part2(&id_ranges)?;

    Ok(())
}

fn part1(id_ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> Result<()> {
    let fresh_ids = ids
        .iter()
        .filter(|id| {
            let res = id_ranges
                .iter()
                .find(|&(min, max)| id >= &min && id <= &max);

            match res {
                Some(_) => true,
                None => false,
            }
        })
        .count();

    println!("{}", fresh_ids);

    Ok(())
}

fn part2(id_ranges: &Vec<(i64, i64)>) -> Result<()> {
    let fresh_ids: i64 = merge_overlapping_ranges(id_ranges)
        .iter()
        .map(|(start, end)| start.abs_diff(*end) as i64 + 1)
        .sum();

    println!("{}", fresh_ids);

    Ok(())
}

fn merge_overlapping_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ranges_copy = ranges.clone();
    ranges_copy.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges_copy[0];

    for &(start, end) in ranges_copy.iter().skip(1) {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    return merged;
}

fn parse_to_int(value: &str) -> Result<i64> {
    let parsed: i64 = value.parse::<i64>()?;

    return Ok(parsed);
}
