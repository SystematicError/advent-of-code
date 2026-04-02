use memoize::memoize;
use std::fs;
use std::sync::OnceLock;

static INPUT: OnceLock<Vec<Vec<char>>> = OnceLock::new();

#[memoize]
fn timelines(i: usize, j: usize) -> u64 {
    let input = INPUT.get().unwrap();

    if i >= input.len() {
        return 1;
    }

    if input[i][j] == '^' {
        return timelines(i, j - 1) + timelines(i, j + 1);
    } else {
        return timelines(i + 1, j);
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day7").unwrap();

    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = input[0].iter().position(|&c| c == 'S').unwrap();

    INPUT.set(input).unwrap();

    println!("There are {} possible timelines", timelines(0, start));
}
