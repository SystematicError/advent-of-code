use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1").unwrap();

    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();

        match dir {
            "L" => dial = (dial - num).rem_euclid(100),
            "R" => dial = (dial + num) % 100,
            _ => {
                panic!("Got unexpected line: {}", line)
            }
        }

        if dial == 0 {
            count += 1;
        }
    }

    println!("The dial hit zero {} times", count);
}
