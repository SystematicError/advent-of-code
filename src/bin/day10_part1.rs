use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

struct Machine {
    lights: HashSet<usize>,
    buttons: Vec<HashSet<usize>>,
}

impl Machine {
    fn new(lights: HashSet<usize>, buttons: Vec<HashSet<usize>>) -> Self {
        Self { lights, buttons }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day10").unwrap();

    let machines: Vec<_> = input
        .lines()
        .map(|line| {
            let (lights, rest) = line.split_once(" ").unwrap();
            let (buttons, _) = rest.rsplit_once(" ").unwrap();

            let lights = lights
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .char_indices()
                .filter(|&(_, c)| c == '#')
                .map(|(i, _)| i)
                .collect();

            let buttons = buttons
                .split(" ")
                .map(|bs| {
                    bs.strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            Machine::new(lights, buttons)
        })
        .collect();

    let mut total = 0;

    for machine in machines {
        'outer: for count in 1..=machine.buttons.len() {
            for to_press in machine.buttons.iter().combinations(count) {
                let mut lights = HashSet::new();

                for button in to_press {
                    lights = lights.symmetric_difference(button).copied().collect();
                }

                if lights == machine.lights {
                    total += count;
                    break 'outer;
                }
            }
        }
    }

    println!("The total is {}", total);
}
