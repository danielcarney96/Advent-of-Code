use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::{ min, max };

type Coordinate = (isize, isize);
type LineBounds = (Coordinate, Coordinate);

fn main() {
    part_one();
    part_two();
}

fn string_to_coordinate(string: &str) -> Coordinate {
    let mut numbers = string.split(',')
        .map(|num| num.trim().parse::<isize>().unwrap());

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

fn part_two() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.splitn(2, "->")
                .map(|coords| string_to_coordinate(coords))
                .collect_tuple::<LineBounds>().unwrap()
        })
        .collect::<Vec<LineBounds>>();
    
    // all the points
    let mut points = HashMap::new();

    for line in lines {
        // Row or column, else diagonal
        if line.0.0 == line.1.0 || line.0.1 == line.1.1 {
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
        } else {
            let x1 = line.0.0;
            let x2 = line.1.0;
            let y1 = line.0.1;
            let y2 = line.1.1;
            
            // Check if the direction is ascending or descending, adjust at end
            let direction = if (x1 < x2) == (y1 < y2) { 1 } else { -1 };
            for i in min(0, x2 - x1)..=max(0, x2 - x1) {
                *points.entry((x1 + i, y1 + i * direction)).or_insert(0) += 1;
            }
        }
    }

    // filter points that occur more than once, then count the total
    let result = points.values().filter(|&&point| point > 1).count();

    println!("{}", result);
}
