use itertools::{iproduct, Itertools};
use petgraph::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::{intcode::IntcodeMachine, util::Direction};

fn dir_to_int(dir: &Direction) -> i64 {
    use Direction::*;
    match *dir {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4,
    }
}

fn surrounding(x: i32, y: i32) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    ((x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Occupancy {
    Unknown,
    Wall,
    Empty,
    Oxygen,
}

impl Default for Occupancy {
    fn default() -> Self {
        Occupancy::Unknown
    }
}

impl From<i64> for Occupancy {
    fn from(i: i64) -> Occupancy {
        match i {
            0 => Occupancy::Wall,
            1 => Occupancy::Empty,
            2 => Occupancy::Oxygen,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Robot {
    program: IntcodeMachine,
    pos: (i32, i32),
}

impl Robot {
    fn step(&mut self, dir: Direction) -> ((i32, i32), Occupancy) {
        let mut input = Some(dir_to_int(&dir));
        let mut out = None;
        self.program.run_while_input(&mut input, &mut out);
        let occ = out.unwrap().into();
        let mov = dir.tuple();
        let new = (self.pos.0 + mov.0, self.pos.1 + mov.1);
        match occ {
            Occupancy::Wall => (),
            _ => {
                // if not wall, move
                self.pos = new;
            }
        }
        (new, occ)
    }
}

struct RobotMemory {
    graph: UnGraph<Occupancy, ()>,
    idx_map: HashMap<(i32, i32), NodeIndex>,
    oxygen_pos: Option<(i32, i32)>,
}

impl RobotMemory {
    fn new(size: i32) -> Self {
        let mut graph = UnGraph::new_undirected();
        let mut map = HashMap::new();
        for (x, y) in iproduct!(0..size, 0..size) {
            let n = graph.add_node(Occupancy::Unknown);
            map.insert((x, y), n);
        }
        // initialize the starting position as Empty
        graph[map[&(size / 2, size / 2)]] = Occupancy::Empty;
        RobotMemory {
            graph,
            idx_map: map,
            oxygen_pos: None,
        }
    }

    fn get(&self, pos: (i32, i32)) -> Occupancy {
        self.graph[self.idx_map[&pos]]
    }

    fn get_oxygen_node(&self) -> Option<NodeIndex> {
        self.oxygen_pos.map(|p| self.idx_map[&p])
    }

    fn connect(&mut self, pos: (i32, i32), occupancy: Occupancy) {
        if let Occupancy::Oxygen = occupancy {
            self.oxygen_pos = Some(pos);
        }
        let (u, d, l, r) = surrounding(pos.0, pos.1);
        let curr = self.idx_map[&pos];
        self.graph[curr] = occupancy;
        let surrounding = [
            self.idx_map[&u],
            self.idx_map[&d],
            self.idx_map[&l],
            self.idx_map[&r],
        ];
        let mut edges = Vec::with_capacity(4);
        for n in surrounding.into_iter() {
            match self.graph[*n] {
                Occupancy::Wall | Occupancy::Unknown => continue,
                _ => edges.push((curr, *n)),
            }
        }
        self.graph.extend_with_edges(edges);
    }

    fn surrounding_occupancy(
        &self,
        pos: (i32, i32),
    ) -> (Occupancy, Occupancy, Occupancy, Occupancy) {
        let (u, d, l, r) = surrounding(pos.0, pos.1);
        (
            self.graph[self.idx_map[&u]],
            self.graph[self.idx_map[&d]],
            self.graph[self.idx_map[&l]],
            self.graph[self.idx_map[&r]],
        )
    }
}

fn run_robots(program: IntcodeMachine) -> RobotMemory {
        // creating a suitably large memory -- the size of the area is unknown
    // but it's assumed to not be incredibly large
    let mut map_memory = RobotMemory::new(50);
    let origin = Robot {
        program: program.clone(),
        pos: (25, 25),
    };

    let mut queue = VecDeque::with_capacity(4);
    queue.push_back(origin);
    // let mut out_buffer = None;
    use Direction::*;
    let dirs = [Up, Down, Left, Right];
    while let Some(robot) = queue.pop_front() {
        let (u, d, l, r) = map_memory.surrounding_occupancy(robot.pos);
        for (&dir, neighbor) in dirs.iter().zip([u, d, l, r].into_iter()) {
            // only walk into unknown territory
            if let Occupancy::Unknown = neighbor {
                let mut new = robot.clone();
                let (pos, occ) = new.step(dir);
                map_memory.connect(pos, occ);
                match occ {
                    Occupancy::Wall => continue,
                    _ => (),
                }
                // new robot will spawn more
                queue.push_back(new);
            }
        }
    }
    map_memory

}
pub fn part1(input: &str) -> u32 {
    let program = IntcodeMachine::from_str(input);
    let memory = run_robots(program);
    let start = memory.idx_map[&(25,25)];
    let end = memory.get_oxygen_node().unwrap();
    petgraph::algo::astar(&memory.graph, start, |n| n == end, |_| 1, |_| 0).unwrap().0
}

pub fn part2(input: &str) -> u32 {
    let program = IntcodeMachine::from_str(input);
    let memory = run_robots(program);
    let oxygen = memory.get_oxygen_node().unwrap();
    let dists = petgraph::algo::dijkstra(&memory.graph, oxygen, None, |_| 1);
    *dists.values().max().unwrap()
}
