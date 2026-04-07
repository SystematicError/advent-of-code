use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;

fn count_paths(
    graph: &DiGraph<&str, ()>,
    current: NodeIndex,
    goal: NodeIndex,
    memo: &mut HashMap<NodeIndex, u64>,
) -> u64 {
    if current == goal {
        return 1;
    }

    if let Some(&count) = memo.get(&current) {
        return count;
    }

    let mut total = 0;

    for neighbor in graph.neighbors(current) {
        total += count_paths(graph, neighbor, goal, memo);
    }

    memo.insert(current, total);

    total
}

fn main() {
    let input = fs::read_to_string("inputs/day11").unwrap();

    let mut graph: DiGraph<&str, ()> = DiGraph::new();

    let adjacency_list: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (from, rest) = line.split_once(": ").unwrap();
            let tos: Vec<_> = rest.split(" ").collect();

            (graph.add_node(from), tos)
        })
        .collect();

    let svr = graph.node_indices().find(|&i| graph[i] == "svr").unwrap();
    let out = graph.add_node("out");

    let dac = graph.node_indices().find(|&i| graph[i] == "dac").unwrap();
    let fft = graph.node_indices().find(|&i| graph[i] == "fft").unwrap();

    for (from, tos) in adjacency_list {
        for to in tos {
            let to = graph.node_indices().find(|&i| graph[i] == to).unwrap();
            graph.add_edge(from, to, ());
        }
    }

    let srv_to_dac = count_paths(&graph, svr, dac, &mut HashMap::new());
    let dac_to_fft = count_paths(&graph, dac, fft, &mut HashMap::new());
    let fft_to_out = count_paths(&graph, fft, out, &mut HashMap::new());

    let srv_to_fft = count_paths(&graph, svr, fft, &mut HashMap::new());
    let fft_to_dac = count_paths(&graph, fft, dac, &mut HashMap::new());
    let dac_to_out = count_paths(&graph, dac, out, &mut HashMap::new());

    let total = srv_to_dac * dac_to_fft * fft_to_out + srv_to_fft * fft_to_dac * dac_to_out;

    println!("The total is {}", total);
}
