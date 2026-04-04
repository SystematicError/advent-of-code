use itertools::Itertools;
use std::fs;

fn rect_area(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)) as u64
}

fn main() {
    let input = fs::read_to_string("inputs/day9").unwrap();

    let coords: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let area = coords
        .into_iter()
        .combinations(2)
        .map(|c| rect_area(c[0], c[1]))
        .max()
        .unwrap();

    println!("The largest area is {}", area);
}
