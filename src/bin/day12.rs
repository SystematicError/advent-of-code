use std::fs;

// The actual packing problem is quite complicated to solve, however the intended solution to this
// puzzle kinda cheeses it by using a heuristic. Unfortunately, this works on the actual puzzle but
// not on the test case, which is quite frustrating (probably not as frustrating as solving this
// the "hard" way tho :P)

fn main() {
    let input = fs::read_to_string("inputs/day12").unwrap();

    let total = input
        .rsplit_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let (dimension, counts) = line.split_once(": ").unwrap();

            let (x, y) = dimension.split_once("x").unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();

            let counts_sum: u32 = counts.split(" ").map(|n| n.parse::<u32>().unwrap()).sum();

            (x / 3) * (y / 3) >= counts_sum
        })
        .filter(|&x| x)
        .count();

    println!("The total is {}", total)
}
