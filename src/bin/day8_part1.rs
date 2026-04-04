use petgraph::algo::tarjan_scc;
use petgraph::graph::UnGraph;
use std::fs;

fn dist(p1: (u32, u32, u32), p2: (u32, u32, u32)) -> f64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;

    let dx = x2 as f64 - x1 as f64;
    let dy = y2 as f64 - y1 as f64;
    let dz = z2 as f64 - z1 as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn main() {
    let input = fs::read_to_string("inputs/day8").unwrap();

    let boxes: Vec<(u32, u32, u32)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| {
            let mut nums = l.split(",").map(|n| n.parse().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect();

    let mut edges: Vec<(usize, usize)> = (0..boxes.len())
        .flat_map(|i| (i + 1..boxes.len()).map(move |j| (i, j)))
        .collect();

    edges.sort_by(|&(a_i, a_j), &(b_i, b_j)| {
        let da = dist(boxes[a_i], boxes[a_j]);
        let db = dist(boxes[b_i], boxes[b_j]);

        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    });

    let graph =
        UnGraph::<usize, ()>::from_edges(edges[0..1000].iter().map(|&(u, v)| (u as u32, v as u32)));

    let mut components = tarjan_scc(&graph);

    components.sort_by(|a, b| b.len().cmp(&a.len()));

    let total = components
        .iter()
        .take(3)
        .map(|l| l.len())
        .fold(1, |acc, x| acc * x);

    println!("The total is {}", total);
}
