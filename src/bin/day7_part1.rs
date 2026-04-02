use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day7").unwrap();

    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(input.remove(0).into_iter().position(|c| c == 'S').unwrap());

    let mut split_count = 0;

    for line in input {
        let mut split_beams: HashSet<usize> = HashSet::new();

        for &beam in &beams {
            if line[beam] == '^' {
                split_beams.insert(beam - 1);
                split_beams.insert(beam + 1);
                split_count += 1;
            } else {
                split_beams.insert(beam);
            }
        }

        beams = split_beams;
    }

    println!("Tachyon split {} times", split_count);
}
