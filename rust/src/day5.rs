//! Day 5: Sunny with a Chance of Asteroids
//!
//! # Problem Description
//!
//! Second day dealing with the Intcode machine. This time we had to
//! expand it with 4 additional operations and an extra mode for
//! operation parameters. The actual day-specific task was to run a
//! test program to make sure your implementation was fully working
//! properly.
//!
//! #Implementation Details
//!
//! This is when I moved the intcode stuff to a separate model which
//! can be day-agnostic. See [intcode] for more information.

use super::intcode;

pub fn part1(input: &str) -> i64 {
    let mut codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //let stdin = std::io::stdin();
    //let input = stdin.lock().lines();    
    intcode::run_program(&mut codes, &mut "1", &mut std::io::sink());
    codes[0]
}

pub fn part2(input: &str) -> isize {
    let mut codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //let stdin = std::io::stdin();
    //let input = stdin.lock().lines();
    intcode::run_program(&mut codes, &mut "5", &mut std::io::sink());
    codes[0]
}
