use std::process;

fn main() {
    part_one();
    part_two();
}

type Card = Vec<Vec<usize>>;

// Function to check if a given card has a full row or column of numbers in an array
fn has_card_won(numbers: &[usize], card: &Card) -> Option<usize> {
    for i in 0..5 {
        // Check rows
        if (0..5).all(|n| numbers.contains(&card[i][n])) {
            let last_number = numbers.last().unwrap();
            let unmarked: usize = card.iter()
                .flatten()
                .filter(|number| !numbers.contains(number))
                .sum();

            return Some(last_number * unmarked);
        }

        // Check columns
        if (0..5).all(|n| numbers.contains(&card[n][i])) {
            let last_number = numbers.last().unwrap();
            let unmarked: usize = card.iter()
                .flatten()
                .filter(|number| !numbers.contains(number))
                .sum();

            return Some(last_number * unmarked);
        }
    }

    return None;
}

fn part_one() {
    // Grab the first line and convert into a vector of numbers
    let draw_numbers: Vec<usize> = include_str!("../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .replace(",", " ")
        .as_str()
        .split_whitespace()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    // Grab lines after the first line split into cards, map into a grid structure
    let bingo_cards: Vec<Card> = include_str!("../input.txt")
        .split("\n\n")
        .skip(1)
        .map(|board| {
            board.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.trim().parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // Loop once for each number in the draw
    for i in 0..draw_numbers.len() {
        // For each draw number, check the bingo cards in order

        let winner = bingo_cards.iter().find_map(|card| has_card_won(&draw_numbers[0..i], card));

        match winner {
            Some(result) => {
                println!("{}", result);
                break;
            },
            _ => {},
        }
    }
}

fn part_two() {
    // Grab the first line and convert into a vector of numbers
    let draw_numbers: Vec<usize> = include_str!("../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .replace(",", " ")
        .as_str()
        .split_whitespace()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    // Grab lines after the first line split into cards, map into a grid structure
    let mut bingo_cards: Vec<Card> = include_str!("../input.txt")
        .split("\n\n")
        .skip(1)
        .map(|board| {
            board.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.trim().parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // Loop once for each number in the draw
    for i in 0..draw_numbers.len() {
        // For each draw number, check the bingo cards in order

        // Collect all winners along with their card
        let winners: Vec<_> = bingo_cards.iter()
            .filter_map(|card| {
                has_card_won(&draw_numbers[0..i], &card)
                    .map(|result| (card.clone(), result))
            })
            .collect();

        // Loop each winner
        for (card, result) in &winners {
            // If there's more than 1 card left its safe to remove
            if bingo_cards.len() > 1 {
                let index = &bingo_cards.iter().position(|i| i == card).unwrap();

                // Remove the card so they can't win more than once
                bingo_cards.remove(*index);
            } else {
                // Last card, return result
                println!("{}", result);
                process::exit(1);
            }
        }
    }
}
