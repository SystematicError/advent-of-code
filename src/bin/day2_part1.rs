use std::fs;

fn is_invalid(n: u64) -> bool {
    let n_str = n.to_string();

    let half_index = n_str.len() / 2;

    &n_str[half_index..] == &n_str[..half_index]
}

fn main() {
    let input = fs::read_to_string("inputs/day2").unwrap();

    let mut sum = 0;

    for line in input.split(",") {
        let (start, end) = line.trim().split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for n in start..(end + 1) {
            if is_invalid(n) {
                sum += n;
            }
        }
    }

    println!("Sum of invalid IDs: {}", sum);
}
