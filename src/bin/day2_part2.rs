use std::fs;

fn is_invalid(n: u64) -> bool {
    let n_str = n.to_string();
    let n_len = n_str.len();

    for chunk_len in 1..=(n_len / 2) {
        if n_len % chunk_len == 0 {
            let chunk = &n_str[0..chunk_len];

            // Check if the entire string is just this chunk repeated
            let mut is_repeat = true;

            for i in (chunk_len..n_len).step_by(chunk_len) {
                if &n_str[i..i + chunk_len] != chunk {
                    is_repeat = false;
                    break;
                }
            }

            if is_repeat {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("inputs/day2").unwrap();

    let mut sum = 0;

    for line in input.split(",") {
        let (start, end) = line.trim().split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for n in start..=end {
            if is_invalid(n) {
                sum += n;
            }
        }
    }

    println!("Sum of invalid IDs: {}", sum);
}
