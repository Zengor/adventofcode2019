use itertools::Itertools;

use crate::intcode;

pub fn part1(input: &str) -> usize {
    let mut output: Vec<i64> = Vec::new();
    intcode::run_program_from_str(input, &mut std::io::empty(), &mut output);
    output
        .into_iter()
        .tuples()
        .filter(|(_, _, t)| *t == 2)
        .count()
}

/// Modifies the game source code so that every tile on the
/// same height as the paddle is also a paddle
fn hack(mut source: Vec<i64>) -> Vec<i64> {
    const EMPTY: i64 = 0;
    const WALL: i64 = 1;
    const PADDLE: i64 = 3;
    // the tiles are just explicitly laid out in the code without any
    // form of encryption, so we just need to find the paddle and fill
    // all tiles up until the walls at the same eight
    let mut paddle_found = false;
    let mut prev_wall = 0;
    let mut after_wall = 0;
    for i in 0..source.len() {
        // paddle starts surrounded by EMPTY. This check is necessary to not confuse
        // the tile section of the code with any other random `3`
        if source[i] == PADDLE && source[i - 1] == EMPTY && source[i + 1] == EMPTY {
            paddle_found = true;
        }
        if source[i] == WALL {
            prev_wall = after_wall;
            after_wall = i;
            if paddle_found {
                // we have seen the paddle as well as the walls before and after it
                break;
            }
        }
    }
    // there should be a paddle at this point
    for i in prev_wall+1..=after_wall-1 {
        source[i] = PADDLE;
    }
    source
}

pub fn part2(input: &str) -> i64 {
    let mut source_code: Vec<_> = input
        .trim()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    source_code[0] = 2;
    let mut game = intcode::IntcodeMachine::copy_program(&hack(source_code));
    let mut out_buffer: Vec<i64> = Vec::new();
    let mut game_input = std::iter::repeat(0);
    while !game.is_stopped() {
        out_buffer.clear();
        game.run_while_input(&mut game_input, &mut out_buffer);        
    }
    // score should be the last number outputted
    return *out_buffer.last().unwrap()
}
