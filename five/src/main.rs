use itertools::Itertools;
use std::collections::HashMap;

type Coordinate = (usize, usize);
type LineBounds = (Coordinate, Coordinate);

fn main() {
    part_one();
}

fn string_to_coordinate(string: &str) -> Coordinate {
    let mut numbers = string.split(',')
        .map(|num| num.trim().parse::<usize>().unwrap());

    return (numbers.next().unwrap(), numbers.next().unwrap());
}

fn part_one() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.splitn(2, "->")
                .map(|coords| string_to_coordinate(coords))
                .collect_tuple::<LineBounds>().unwrap()
        })
        .collect::<Vec<LineBounds>>()
        .iter()
        .copied()
        .filter(|bounds| bounds.0.0 == bounds.1.0 || bounds.0.1 == bounds.1.1) // filter out diagonals
        .collect::<Vec<LineBounds>>();
    
    // all the points
    let mut points = HashMap::new();

    for line in lines {
        // x1 and y1 should be the smallest so that the range works
        let x1 = if line.0.0 < line.1.0 { line.0.0 } else { line.1.0 };
        let x2 = if line.1.0 < line.0.0 { line.0.0 } else { line.1.0 };
        let y1 = if line.0.1 < line.1.1 { line.0.1 } else { line.1.1 };
        let y2 = if line.1.1 < line.0.1 { line.0.1 } else { line.1.1 };

        for x in x1..x2+1 {
            for y in y1..y2+1 {
                *points.entry((x,y)).or_insert(0) += 1;
            }
        }
    }

    // filter points that occur more than once, then count the total
    let result = points.values().filter(|&&point| point > 1).count();

    println!("{}", result);
}
