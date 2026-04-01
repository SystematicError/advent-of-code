use std::fs;

fn max_joltage(digits: &[u32], target: u32) -> u32 {
    if target == 1 {
        return *digits.iter().max().unwrap();
    }

    let first = *digits[..digits.len() - (target as usize - 1)]
        .iter()
        .max()
        .unwrap();

    let i = digits.iter().position(|&x| x == first).unwrap();

    10 * first + max_joltage(&digits[i + 1..], target - 1)
}

fn main() {
    let input = fs::read_to_string("inputs/day3").unwrap();

    let jolatage_sum = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|d| max_joltage(&d, 2))
        .sum::<u32>();

    println!("The sum of the battery joltages is {}", jolatage_sum)
}
