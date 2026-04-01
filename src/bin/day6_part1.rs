use std::fs;

fn evaluate_column(operator: char, numbers: Vec<u64>) -> u64 {
    match operator {
        '+' => numbers.iter().fold(0, |acc, x| acc + x),
        '*' => numbers.iter().fold(1, |acc, x| acc * x),
        _ => panic!("Unexpected operator"),
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day6").unwrap();

    let mut numbers: Vec<&str> = input.lines().collect();

    let operators: Vec<char> = numbers
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let numbers: Vec<Vec<u64>> = numbers
        .iter()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let columns: Vec<(char, Vec<u64>)> = operators
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            let nums = numbers.iter().map(|row| row[i]).collect();
            (op, nums)
        })
        .collect();

    let total: u64 = columns
        .into_iter()
        .map(|(op, nums)| evaluate_column(op, nums))
        .sum();

    println!("The total is {}", total);
}
