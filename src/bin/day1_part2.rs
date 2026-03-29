use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1").unwrap();

    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();

        let rot = match dir {
            "L" => -num,
            "R" => num,
            _ => panic!("Got unexpected line: {}", line),
        };

        let dial_unwrapped = dial + rot;

        let mut rotations = (dial_unwrapped / 100).abs();

        if dial != 0 && dial_unwrapped <= 0 {
            rotations += 1;
        }

        count += rotations;

        dial = dial_unwrapped.rem_euclid(100);
    }

    println!("The dial hit or crossed zero {} times", count);
}
