fn main() {
    part_one();
}

pub fn part_one() {
    let mut sums: [u32; 12] = [0; 12];

    let lines: Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();

    let line_count = &lines.len();

    for line in &lines {
        for (index, character) in line.chars().enumerate() {
            sums[index] += character.to_digit(10).unwrap();
        }
    }

    let mut gamma: String = String::new();

    for sum in sums {
        if sum as usize >= line_count / 2 {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    let gamma_int = usize::from_str_radix(&gamma[..], 2).unwrap();
    
    println!("gamma {}", gamma_int * (!gamma_int & 0xfff));
}
