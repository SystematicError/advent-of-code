use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::unionfind::UnionFind;
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

    let mut graph = UnGraph::<usize, ()>::new_undirected();

    let node_indices: Vec<NodeIndex> = (0..boxes.len()).map(|i| graph.add_node(i)).collect();

    for pos in 0..boxes.len() {
        graph.add_node(pos);
    }

    // for (u, v) in edges {
    //     graph.add_edge(node_indices[u], node_indices[v], ());
    //
    //     if connected_components(&graph) == 1 {
    //         break;
    //     }
    // }

    let mut vertex_sets = UnionFind::new(boxes.len());
    let mut components = boxes.len(); // Start with every node in its own component

    for (from, to) in edges {
        if !vertex_sets.equiv(from, to) {
            vertex_sets.union(from, to);
            graph.add_edge(node_indices[from], node_indices[to], ());
            components -= 1;
        }

        if components == 1 {
            println!("The total is {}", boxes[from].0 * boxes[to].0);
            break;
        }
    }
}
