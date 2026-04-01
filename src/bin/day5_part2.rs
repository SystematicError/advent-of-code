use std::cmp;
use std::fs;

fn possible_ids(mut ranges: Vec<(u64, u64)>) -> u64 {
    ranges.sort_unstable_by_key(|&range| range.0);

    let mut ranges = ranges.into_iter();

    let mut merged: Vec<(u64, u64)> = vec![ranges.next().unwrap()];

    for range in ranges {
        let prev = merged.last_mut().unwrap();

        if prev.1 < range.0 {
            merged.push(range)
        } else {
            prev.1 = cmp::max(prev.1, range.1);
        }
    }

    merged.iter().map(|range| range.1 - range.0 + 1).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/day5").unwrap();

    let ranges = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|l| {
            let (x, y) = l.split_once("-").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let count = possible_ids(ranges);

    println!("There are {} valid fresh ingredient IDs", count);
}
