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
    part2(&input)?;

    Ok(())
}

fn part1(id_ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> Result<()> {
    let fresh_ids: i64 = ids
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
        .map(|_| 1)
        .sum();

    println!("{}", fresh_ids);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}

fn parse_to_int(value: &str) -> Result<i64> {
    let parsed: i64 = value.parse::<i64>()?;

    return Ok(parsed);
}
