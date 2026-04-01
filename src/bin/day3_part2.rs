use std::fs;

// Experimented a bit here, but the code is kinda ugly

fn max_joltage(digits: &[u64], target: u64) -> u64 {
    if target == 1 {
        return *digits.iter().max().unwrap();
    }

    let first = *digits[..digits.len() - (target as usize - 1)]
        .iter()
        .max()
        .unwrap();

    let i = digits.iter().position(|&x| x == first).unwrap();

    (10 as u64).pow((target - 1) as u32) * first + max_joltage(&digits[i + 1..], target - 1)
}

fn main() {
    let input = fs::read_to_string("inputs/day3").unwrap();

    let jolatage_sum = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .map(|d| max_joltage(&d, 12))
        .sum::<u64>();

    println!("The sum of the battery joltages is {}", jolatage_sum)
}
