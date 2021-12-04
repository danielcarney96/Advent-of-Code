use std::process;

fn main() {
    part_one();
}

type Card = Vec<Vec<usize>>;

// Function to check if a given card has a full row or column of numbers in an array
fn has_card_won(numbers: &[usize], card: &Card) {
    for i in 0..5 {
        // Check rows
        if (0..5).all(|n| numbers.contains(&card[i][n])) {
            let last_number = numbers.last().unwrap();
            let unmarked: usize = card.iter()
                .flatten()
                .filter(|number| !numbers.contains(number))
                .sum();

            println!("{}", last_number * unmarked);
            process::exit(1);
        }

        // Check columns
        if (0..5).all(|n| numbers.contains(&card[n][i])) {
            let last_number = numbers.last().unwrap();
            let unmarked: usize = card.iter()
                .flatten()
                .filter(|number| !numbers.contains(number))
                .sum();

            println!("{}", last_number * unmarked);
            process::exit(1);
        }
    }
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
        for card in &bingo_cards {
            // Pass the drawn numbers up to the index, used to check new number along with any old ones
            has_card_won(&draw_numbers[0..i], card);
        }
    }
}
