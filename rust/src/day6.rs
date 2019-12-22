//! Day 6: Universal Orbit Map
//!
//! # Problem Description
//!
//! For part 1: given a list of orbital relationships (B orbits A),
//! find the total number of "direct" and "indirect" orbits. That is,
//! in a sequence (C orbits B, which orbits A), B directly orbits A; C
//! directly orbits B and indirectly orbits A, for a total of 3
//! orbits.
//!
//! In part 2, we are just supposed to find the shortest path between
//! two nodes.
//!
//! #Implementation Details
//!
//! While this can be done with a tree, I implemented this as a graph
//! using [`petgraph`](https://docs.rs/petgraph/0.4.13/petgraph/).
//! This would have been quicker and simpler with a makeshift
//! tree/graph implementation but I wanted to practice with `petgraph`
//! as I had never used it before and think it might be useful in the
//! future (not only for AOC).
//!
//! The hardest part of using petgraph was actually understanding how
//! to build the graph. Other than that, it was hard to figure out
//! which particular way the search for part 2 should be done, as it
//! provides many different ways to do basic BFS/DFS without really
//! indicating which is best in which situation.
//!
//! For part 1, the solution is just to do a topological sort (with
//! the implementation given in `petgraph`, which I assume is just a
//! DFS) and then walk backwards through the graph, storing the
//! number of orbits of each planet in order. That way, when getting
//! to planet C, which orbits B, the number of orbits for planet B is
//! already known, and can just be added to the count for planet C.
//!
//! If done as a tree, I'd have just stored depth information for each
//! node, which would already indicate its number of orbits.
//!
//! Part 2 turned out to be an advantage of doing this as a graph as
//! all that had to be done was a BFS with an undirected version of
//! the graph. As a tree, I would have done a recursive climb through
//! the tree to find the common parent of the two nodes. The sum of
//! the depth differences between the parent and the nodes is the
//! distance between node YOU and SAN.

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
    let graph = parse_graphmap::<petgraph::Directed>(input);
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
