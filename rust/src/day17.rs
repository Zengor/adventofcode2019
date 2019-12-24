use crate::{intcode::{IntcodeMachine, run_from_str}, util::Matrix};
use itertools::iproduct;

fn surrounding(
    x: usize,
    y: usize,
) -> (
    (usize, usize),
    (usize, usize),
    (usize, usize),
    (usize, usize),
) {
    ((x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y))
}

const ROPE: i64 = '#' as i64;
const NEWLINE: i64 = '\n' as i64;

pub fn part1(input: &str) -> usize {
    let mut screen = Vec::with_capacity(3000);
    run_from_str(input, &mut std::io::empty(), &mut screen);
    let line_width = screen.iter().position(|c| *c == NEWLINE).unwrap() + 1;
    let grid = Matrix::wrap(screen, line_width);
    let mut alignment_sum = 0;
    for (y, x) in iproduct!(1..grid.width() - 1, 1..grid.height() - 1) {
        if grid[(x, y)] == ROPE {
            let (u, d, l, r) = surrounding(x, y);
            if grid[u] == ROPE && grid[d] == ROPE && grid[l] == ROPE && grid[r] == ROPE {
                alignment_sum += x * y;
            }
        }
    }
    alignment_sum
}

pub fn part2(input: &str) -> i64 {
    let mut source_code: Vec<_> = input
        .trim()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    source_code[0] = 2;
    let mut machine = IntcodeMachine::copy_program(&source_code);
    let a_routine = "L,10,R,8,L,6,R,6\n";
    let b_routine = "L,8,L,8,R,8\n";
    let c_routine = "R,8,L,6,L,10,L,10\n";
    let main_routine = "A,B,A,C,A,B,C,B,C,B\n";
    let mut full = String::new();
    full += main_routine;
    full += a_routine;
    full += b_routine;
    full += c_routine;
    full += "n\n";
    let mut robot_input: Vec<i64> = full.chars().map(|c| c as i64).collect();
    let mut out = Vec::new();
    machine.run(&mut robot_input, &mut out);
    *out.last().unwrap()
}
