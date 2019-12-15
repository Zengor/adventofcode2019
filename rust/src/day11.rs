use itertools::Itertools;
use std::collections::HashMap;

use crate::intcode::IntcodeMachine;
use crate::util::Direction;

fn paint(mut program: IntcodeMachine, start_tile: i64) -> HashMap<(i32,i32),i64> {
    let mut pos = (0, 0);
    let mut facing_dir = Direction::Up;
    let mut painted_tiles: HashMap<(i32, i32), i64> = HashMap::new();
    let mut out_buffer = Vec::with_capacity(2);
    let mut input = Some(start_tile);
    while !program.is_stopped() {
        program.run_while_input(&mut input, &mut out_buffer);
        for (&paint, &turn) in out_buffer.iter().tuples() {
            painted_tiles.insert(pos, paint);
            if turn == 0 {
                facing_dir.turn_left();
            } else {
                facing_dir.turn_right();
            };
            let mov = facing_dir.tuple();
            pos = (pos.0 + mov.0, pos.1 + mov.1);
        }
        out_buffer.clear();
        input = Some(*painted_tiles.get(&pos).unwrap_or(&0));
    }
    painted_tiles
}

pub fn part1(input: &str) -> usize {
    let program = IntcodeMachine::from_str(input);
    let painted_tiles = paint(program, 0);
    return painted_tiles.len();
}

pub fn part2(input: &str) {
    let program = IntcodeMachine::from_str(input);
    let painted_tiles = paint(program, 1);
    
    let mut sorted = painted_tiles.keys().sorted_by(|a, b| {
        use std::cmp::Ordering::*;
        match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
            (x, Equal) => x,
            (_, y) => y,
        }
    }).peekable();
    
    let leftmost = *sorted.peek().unwrap();
    for (_, line) in &sorted.group_by(|k| k.1) {
        let mut x = leftmost.0;
        for p in line {
            while x <= p.0 {
                match painted_tiles.get(p).unwrap_or(&0){
                    0 => print!("░"),
                    1 => print!("▓"),
                    _ => panic!(),
                }
                x += 1;
            }
        }
        println!();
    }
}
