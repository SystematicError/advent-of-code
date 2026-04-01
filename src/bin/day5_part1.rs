use std::fs;

fn is_fresh(ranges: &Vec<(u64, u64)>, id: u64) -> bool {
    for (start, end) in ranges {
        if (start..=end).contains(&&id) {
            return true;
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("inputs/day5").unwrap();
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|l| {
            let (x, y) = l.split_once("-").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let ids: Vec<u64> = ids.lines().map(|n| n.parse().unwrap()).collect();

    let count = ids.iter().filter(|&&id| is_fresh(&ranges, id)).count();

    println!("There are {} fresh ingredients", count);
}
