use std::cmp;
use std::fs;

fn get_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;

    for row in &grid[i.saturating_sub(1)..=cmp::min(i + 1, grid.len() - 1)] {
        for val in &row[j.saturating_sub(1)..=cmp::min(j + 1, row.len() - 1)] {
            if val == &'@' {
                count += 1;
            }
        }
    }

    count - 1
}

fn main() {
    let input = fs::read_to_string("inputs/day4").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if val == &'@' {
                if get_neighbors(&grid, i, j) < 4 {
                    count += 1;
                }
            }
        }
    }

    println!("Total number of accessible rolls: {}", count)
}
