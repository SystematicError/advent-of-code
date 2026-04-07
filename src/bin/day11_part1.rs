use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;

fn count_paths(
    graph: &DiGraph<&str, ()>,
    current: NodeIndex,
    goal: NodeIndex,
    memo: &mut HashMap<NodeIndex, u32>,
) -> u32 {
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

    let you = graph.node_indices().find(|&i| graph[i] == "you").unwrap();
    let out = graph.add_node("out");

    for (from, tos) in adjacency_list {
        for to in tos {
            let to = graph.node_indices().find(|&i| graph[i] == to).unwrap();
            graph.add_edge(from, to, ());
        }
    }

    let total = count_paths(&graph, you, out, &mut HashMap::new());

    println!("The total is {}", total);
}
