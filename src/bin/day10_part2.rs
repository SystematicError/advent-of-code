use microlp::{ComparisonOp, OptimizationDirection, Problem};
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Machine {
    buttons: Vec<HashSet<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn new(buttons: Vec<HashSet<usize>>, joltages: Vec<usize>) -> Self {
        Self { buttons, joltages }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day10").unwrap();

    let machines: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(" ").unwrap();
            let (buttons, joltages) = rest.rsplit_once(" ").unwrap();

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

            let joltages = joltages
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();

            Machine::new(buttons, joltages)
        })
        .collect();

    let mut total = 0;

    for machine in machines {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let max_joltage = machine.joltages.iter().copied().max().unwrap() as i32;

        let vars: Vec<_> = (0..machine.buttons.len())
            .map(|_| problem.add_integer_var(1.0, (0, max_joltage)))
            .collect();

        for (i, &joltage) in machine.joltages.iter().enumerate() {
            let mut coeffs = Vec::new();

            for (b, button) in machine.buttons.iter().enumerate() {
                if button.contains(&i) {
                    coeffs.push((vars[b], 1.0));
                }
            }

            problem.add_constraint(&coeffs, ComparisonOp::Eq, joltage as f64);
        }

        total += problem.solve().unwrap().objective().round() as usize;
    }

    println!("The total is {}", total);
}
