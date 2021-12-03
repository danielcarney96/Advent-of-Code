fn main() {
    part_one();
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
