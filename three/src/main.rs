fn main() {
    part_one();
    part_two();
}

fn most_common(lines: &Vec<u32>, index: usize) -> u32 {
    // Keep a track of 0 and 1 counts
    let mut sums = [0, 0];

    /*
     * Loop each line and shift based on the index.
     * Get a specific column by shifting based on the index and getting
     * the least significant bit using `& 1` e.g.
     *      0001 & 1 -> 1
     *      0000 & 1 -> 0
     * Finally increment the value at the calculated index.
     */
    for &line in lines {
        sums[(line as usize >> index) & 1] += 1;
    }

    // If more 1's return 1, if more 0's return 0
    return (sums[1] >= sums[0]) as u32;
}

fn part_one() {
    // Include the txt file as a string, split into lines and map into u32
    let lines: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    /*
     * Pass the values (0 to 11) to the function `most_common` along with the lines.
     * This returns twelve separate bits that together form the gamma value.
     * Shift each bit by the index to get its integer value and add them together to get the final value.
     */
    let gamma: u32 = (0..12).map(|i| most_common(&lines, i) << i).sum();

    // Finally times gamma by its inverse. We need `0xfff` here to truncate the inverse.
    println!("{}", gamma * (!gamma & 0xfff));
}

fn part_two() {
    // Include the txt file as a string, split into lines and map into u32
    let mut lines: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    // Copy the lines to calculate least common
    let mut lines_2: Vec<u32> = lines.clone();

    // Loop in reverse as the order affects the outcome, `most_common` looks at the last bit first
    for i in (0..12).rev() {
        /**
         * Least common is not always the inverse of common since we use different line
         * vectors for each.
         */
        let common: u32 = most_common(&lines, i);
        let least_common: u32 = most_common(&lines_2, i);

        // Retain any line that matches the most common bit for a given position
        if lines.len() > 1 {
            lines.retain(|line| (line >> i) & 1 == common ^ 1);
        }
        
        if lines_2.len() > 1 {
            lines_2.retain(|line| (line >> i) & 1 == least_common ^ 0);
        }

        // Break if only 1 result is remaining in each vec
        if (lines.len() == 1 && lines_2.len() == 1) {
            break;
        }
    }

    println!("{}", lines[0] * lines_2[0]);
}
