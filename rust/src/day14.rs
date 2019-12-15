use itertools::Itertools;
use petgraph::{algo::toposort, prelude::*};

use std::collections::HashMap;

fn parse_chemical(c: &str) -> (u64, &str) {
    let (n, chem) = c.split(" ").collect_tuple().unwrap();
    (n.parse().unwrap(), chem)
}

struct ReactionEdge {
    produces: u64,
    consumes: u64,
}

impl ReactionEdge {
    fn new(produces: u64, consumes: u64) -> Self {
        Self { produces, consumes }
    }
}

struct Reactions<'a> {
    // Holds the actual dependencies between each node.The data
    // associated with a node is used to calculate needed quantities
    // and starts at 0. The weight of a A->B edge indicates how much
    // B is needed to create a number of A
    deps: DiGraph<u64, ReactionEdge>,
    // this is a pre-computed topological sort so there's no need
    // to find it every time
    sorted: Vec<NodeIndex>,
    // maps the chemical names to their respecitve indices
    chemicals: HashMap<&'a str, NodeIndex>,
}

impl<'a> Reactions<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut deps = DiGraph::new();
        let mut chemicals = HashMap::new();
        for line in input.lines() {
            let (consumes, produces) = line.split("=>").map(|x| x.trim()).collect_tuple().unwrap();
            let produces = parse_chemical(produces);
            let consumes = consumes.split(",").map(|x| parse_chemical(x.trim()));
            let from_node = *chemicals
                .entry(produces.1)
                .or_insert_with(|| deps.add_node(0));
            for c in consumes {
                let to_node = *chemicals.entry(c.1).or_insert_with(|| deps.add_node(0));
                deps.add_edge(from_node, to_node, ReactionEdge::new(produces.0, c.0));
            }
        }
        let sorted = toposort(&deps, None).unwrap();
        Reactions {
            deps,
            sorted,
            chemicals,
        }
    }
    // cleans the required counts for all chemicals
    fn clear_nodes(&mut self) {
        for weight in self.deps.node_weights_mut() {
            *weight = 0;
        }
    }

    fn resources_required_for(&mut self, chem: &str, quantity: u64, resource: &str) -> u64 {
        self.clear_nodes();
        // set required quantity for starting chemical
        let start = self.chemicals[chem];
        self.deps[start] = quantity;
        // in this problem the node we care about (FUEL) should
        // be the first one, but let's just add this to make sure we
        // start with it
        let sorted = self.sorted.iter().skip_while(|n| **n != start);
        for &chem in sorted {
            let mut neighbors = self.deps.neighbors(chem).detach();
            let chem_needed = self.deps[chem];
            while let Some((edge, neighbor)) = neighbors.next(&self.deps) {
                let (produces, consumes) = (self.deps[edge].produces, self.deps[edge].consumes);
                let mut needed_reactions = chem_needed / produces;
                if chem_needed % produces != 0 {
                    // round up
                    needed_reactions += 1;
                }
                self.deps[neighbor] += needed_reactions * consumes;
            }
        }
        self.deps[self.chemicals[resource]]
    }
}

pub fn part1(input: &str) -> u64 {
    let mut reactions = Reactions::from_input(input);
    reactions.resources_required_for("FUEL", 1, "ORE")
}

pub fn part2(input: &str) -> u64 {
    let mut reactions = Reactions::from_input(input);
    let fuel_ore_rate = reactions.resources_required_for("FUEL", 1, "ORE");
    const TRILLION: u64 = 1_000_000_000_000;
    let start_guess = TRILLION / fuel_ore_rate;
    for i in start_guess.. {
        let required = reactions.resources_required_for("FUEL", i, "ORE");
        if required > TRILLION {
            return i - 1;
        }
    }
    unreachable!()
}
