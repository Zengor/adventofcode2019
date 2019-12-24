use bitvec::prelude::*;
use std::collections::{HashMap, HashSet};

type Layout = BitVec<LittleEndian, u32>;

fn apply_sim(grid: &mut Layout, neighbor_counts: &[usize], work_buffer: &mut Layout) -> u32 {
    for i in 0..25 {
        let bug = grid[i];
        let new = if bug && neighbor_counts[i] != 1 {
            false
        } else if !bug && (neighbor_counts[i] == 1 || neighbor_counts[i] == 2) {
            true
        } else {
            bug
        };
        work_buffer.set(i, new);
    }
    grid.clear();
    grid.extend(work_buffer.drain(..));
    // this is safe as we know the memory is allocated and we also
    // won't be reading from it until this same range is
    // overwritten
    unsafe { work_buffer.set_len(25) };
    grid.load().unwrap()
}

fn calc_neighbor_counts(grid: &Layout) -> Vec<usize> {
    let mut counts = Vec::with_capacity(25);
    for i in 0..25 {
        let (x, y) = (i % 5, i / 5);
        let mut count = 0;
        // doing manual bounds checking to avoid underflow or having to
        // use signed integers
        if y > 0 && grid[i - 5] {
            count += 1;
        }
        if y < 4 && grid[i + 5] {
            count += 1;
        }
        if x > 0 && grid[i - 1] {
            count += 1;
        }
        if x < 4 && grid[i + 1] {
            count += 1;
        }
        counts.push(count)
    }
    counts
}

pub fn part1(input: &str) -> u32 {
    let mut grid = BitVec::<_, _>::with_capacity(25);
    for c in input.chars().filter(|c| *c != '\r' && *c != '\n') {
        grid.push(c == '#');
    }

    let mut seen = HashSet::<u32>::new();
    seen.insert(grid.load().unwrap());
    let mut work_buffer = bitvec![LittleEndian, u32; 0; 25];
    loop {
        let counts = calc_neighbor_counts(&grid);
        let rating = apply_sim(&mut grid, &counts, &mut work_buffer);
        if seen.contains(&rating) {
            return rating;
        } else {
            seen.insert(rating);
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut initial = BitVec::<_, _>::with_capacity(25);
    for c in input.chars().filter(|c| *c != '\r' && *c != '\n') {
        initial.push(c == '#');
    }
    let mut grid = RecursiveBugGrid::new(initial);
    for _ in 0..200 {
        grid.sim_step_recursive();
    }
    grid.count_bugs()
}

const UPPER: &[usize] = &[0, 1, 2, 3, 4];
const LOWER: &[usize] = &[20, 21, 22, 23, 24];
const LEFT: &[usize] = &[0, 5, 10, 15, 20];
const RIGHT: &[usize] = &[4, 9, 14, 19, 24];
const INNER_CROSS: &[usize] = &[7, 11, 13, 17];

fn count_bugs(positions: &[usize], grid: &BitSlice<LittleEndian, u32>) -> usize {
    let mut count = 0;
    for &i in positions {
        if grid[i] {
            count += 1
        }
    }
    count
}

#[derive(Debug)]
struct RecursiveBugGrid {
    depths: HashMap<i32, BitVec<LittleEndian, u32>>,
    work_buffer: BitVec<LittleEndian, u32>,
    outermost: i32,
    innermost: i32,
}

impl RecursiveBugGrid {
    fn new(start: BitVec<LittleEndian, u32>) -> Self {
        let level_zero = start.into();
        let mut depths = HashMap::with_capacity(1);
        depths.insert(0, level_zero);
        Self {
            depths,
            work_buffer: bitvec![LittleEndian, u32; 0; 25],
            outermost: 0,
            innermost: 0,
        }
    }

    fn get_at(&self, depth: i32) -> &BitSlice<LittleEndian, u32> {
        &self.depths[&depth]
    }

    fn outer_spread(&self) -> bool {
        let outer = self.get_at(self.outermost);
        for edge in [UPPER, LEFT, RIGHT, LOWER].into_iter() {
            let count = count_bugs(edge, outer);
            if count == 1 || count == 2 {
                return true;
            }
        }
        false
    }

    fn inner_spread(&self) -> bool {
        let inner = self.get_at(self.innermost);
        // if there is any tile with a bug in the innermost inner cross,
        // then that means at least one whole edge at the lower level is
        // neighboring a single bug, meaning it will spread
        INNER_CROSS.iter().map(|i| inner[*i]).any(|i| i)
    }

    fn calc_neighbor_counts_recursive(&self, depth: i32) -> Vec<usize> {
        let grid = self.get_at(depth);
        let mut counts = vec![0; 25];
        for i in (0..=11).chain(13..25) {
            let (x, y) = (i % 5, i / 5);
            let mut count = 0;
            // additional checks if not the outermost or innermost
            if self.outermost != depth {
                let outer = self.get_at(depth - 1);
                if y == 0 && outer[7] {
                    count += 1;
                }
                if y == 4 && outer[17] {
                    count += 1;
                }
                if x == 0 && outer[11] {
                    count += 1;
                }
                if x == 4 && outer[13] {
                    count += 1;
                }
            }
            if self.innermost != depth {
                let inner = self.get_at(depth + 1);
                if i == 7 {
                    count += count_bugs(UPPER, &inner);
                }
                if i == 11 {
                    count += count_bugs(LEFT, &inner);
                }
                if i == 13 {
                    count += count_bugs(RIGHT, &inner);
                }
                if i == 17 {
                    count += count_bugs(LOWER, &inner);
                }
            }
            if y > 0 && grid[i - 5] {
                count += 1;
            }
            if y < 4 && grid[i + 5] {
                count += 1;
            }
            if x > 0 && grid[i - 1] {
                count += 1;
            }
            if x < 4 && grid[i + 1] {
                count += 1;
            }
            counts[i] = count;
        }
        counts
    }

    fn apply_sim(&mut self, depth: i32, neighbor_counts: &[usize]) -> u32 {
        let grid = self.depths.get_mut(&depth).unwrap();
        for i in 0..25 {
            let bug = grid[i];
            let new = if bug && neighbor_counts[i] != 1 {
                false
            } else if !bug && (neighbor_counts[i] == 1 || neighbor_counts[i] == 2) {
                true
            } else {
                bug
            };
            self.work_buffer.set(i, new);
        }
        grid.clear();
        grid.extend(self.work_buffer.drain(..));
        // this is safe as we know the memory is allocated and we also
        // won't be reading from it until this same range is
        // overwritten
        unsafe { self.work_buffer.set_len(25) };
        grid.load().unwrap()
    }

    fn sim_step_recursive(&mut self) {
        if self.outer_spread() {
            self.outermost -= 1;
            self.depths
                .insert(self.outermost, bitvec![LittleEndian, u32; 0; 25]);
        }
        if self.inner_spread() {
            self.innermost += 1;
            self.depths
                .insert(self.innermost, bitvec![LittleEndian, u32; 0; 25]);
        }
        let depths: Vec<_> = self.depths.keys().cloned().collect();
        //println!("{}", depths.len());
        let mut counts_per_depth = HashMap::with_capacity(depths.len());
        for depth in depths.iter() {
            counts_per_depth.insert(depth, self.calc_neighbor_counts_recursive(*depth));
        }
        for depth in depths.iter() {
            self.apply_sim(*depth, &counts_per_depth[depth]);
        }
    }

    fn count_bugs(&self) -> usize {
        self.depths
            .values()
            .map(|depth| depth.count_ones())
            .sum::<usize>()
    }
}
