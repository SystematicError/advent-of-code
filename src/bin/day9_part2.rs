use geo::{Contains, Coord, Polygon, Rect};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

// This is a bit slow :\

fn main() {
    let input = fs::read_to_string("inputs/day9").unwrap();

    let coords: Vec<Coord<f64>> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Coord {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let polygon = Polygon::new(coords.clone().into(), vec![]);

    let pairs: Vec<Vec<Coord<f64>>> = coords.into_iter().combinations(2).collect();

    let max_area = pairs
        .par_iter()
        .map(|coord_pair| {
            let p1 = coord_pair[0];
            let p2 = coord_pair[1];

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let rect = Rect::new(Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y });

            if polygon.contains(&rect) {
                ((max_x - min_x) + 1.0) * ((max_y - min_y) + 1.0)
            } else {
                0.0
            }
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);

    println!("The largest area is {}", max_area);
}
