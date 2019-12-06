use itertools::Itertools;
use petgraph::{algo::toposort, prelude::*};

use std::collections::HashMap;

pub fn parse_graph(input: &str) -> (DiGraph<(), ()>, HashMap<&str, NodeIndex>) {
    let mut nodes = HashMap::new();
    let mut graph: DiGraph<_, _> = DiGraph::new();
    for line in input.lines() {
        let (to, from): (&str, &str) = line.trim().split(")").collect_tuple().unwrap();
        let edge = (
            *nodes.entry(from).or_insert_with(|| graph.add_node(())),
            *nodes.entry(to).or_insert_with(|| graph.add_node(())),
        );
        graph.add_edge(edge.0, edge.1, ());
    }
    (graph, nodes)
}

pub fn parse_graphmap<Ty>(input: &str) -> GraphMap<&str, (), Ty>
where
    Ty: petgraph::EdgeType,
{
    GraphMap::<&str, (), Ty>::from_edges(input.lines().map(|line| {
        line.trim()
            .split(")")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
    }))
}

pub fn part1(input: &str) -> u32 {
    let (graph, _) = parse_graph(input);
    let sorted = toposort(&graph, None).unwrap();
    let orbit_counts =
        sorted
            .iter()
            .rev()
            .fold(HashMap::<&NodeIndex, u32>::new(), |mut orbits, curr| {
                let orbits_sum = graph.neighbors(*curr).map(|n| 1 + orbits[&n]).sum();
                orbits.insert(curr, orbits_sum);
                orbits
            });
    orbit_counts.values().sum()
}
pub fn part1_graphmap(input: &str) -> u32 {
    let
        graph = parse_graphmap::<petgraph::Directed>(input);    
    let sorted = toposort(&graph, None).unwrap();
    let orbit_counts = sorted
        .iter()
        .rev()
        .fold(HashMap::<&str, u32>::new(), |mut orbits, curr| {
            let orbits_sum = graph.neighbors(*curr).map(|n| 1 + orbits[n]).sum();
            orbits.insert(curr, orbits_sum);
            orbits
        });
    orbit_counts.values().sum()
}

pub fn part2(input: &str) -> u32 {
    let (graph, nodes) = parse_graph(input);
    let graph = graph.into_edge_type::<petgraph::Undirected>();
    let (you, san) = (nodes["YOU"], nodes["SAN"]);
    let mut distances = HashMap::<NodeIndex, u32>::new();
    distances.insert(you, 0);
    let mut bfs = Bfs::new(&graph, you);
    while let Some(node) = bfs.next(&graph) {
        if node == san {
            return distances[&san] - 2;
        }
        let mut neighbors = graph.neighbors_undirected(node).detach();
        while let Some(neighbor) = neighbors.next_node(&graph) {
            if distances.contains_key(&neighbor) {
                continue;
            } else {
                distances.insert(neighbor, distances[&node] + 1);
            }
        }
    }
    0
}

pub fn part2_graphmap(input: &str) -> u32 {
    let graph = parse_graphmap::<petgraph::Directed>(input);
    let (start, end) = ("YOU", "SAN");
    let mut distances = HashMap::<&str, u32>::new();
    distances.insert(start, 0);
    let mut bfs = Bfs::new(&graph, start);
    while let Some(node) = bfs.next(&graph) {
        if node == end {
            return distances[end] - 2;
        }
        let neighbors = graph.neighbors(node);
        for neighbor in neighbors {
            if distances.contains_key(&neighbor) {
                continue;
            } else {
                distances.insert(neighbor, distances[&node] + 1);
            }
        }
    }
    0
}

#[allow(dead_code)]
fn find_key<'a>(index: &NodeIndex, nodes: &HashMap<&'a str, NodeIndex>) -> &'a str {
    for (key, value) in nodes.iter() {
        if value == index {
            return *key;
        }
    }
    unreachable!("No node str corresponding to index - should be impossible");
}
