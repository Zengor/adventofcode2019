//! # Day 2: 1202 Program Alarm
//!
//! ## Problem Description
//!
//! This is the first day to introduce Intcode programs. The problem
//! was basically to implement an Intcode computer supporting 2 basic
//! operations. In part 1 you replace positions 1 and 2 of the program
//! with given numbers to check the value that ends up in position 0
//! after the program halts. Part 2 uses the same idea, but you have
//! to test all combinations of numbers and find which results in a
//! specific number.
//!
//! ## Implementention details
//!
//! Intcode programs show up repeatedly in future days, so I my
//! implementation evolved as more features were needed, and was moved
//! to a separate module. There isn't much to say about this day
//! specifically.


use super::intcode::*;

pub fn part1(input: &str) -> i64 {
    // as the codes are used for indexing and can never be negative, i use usize
    // instead of a signed integer type
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    try_inputs(12, 2, codes)
}

pub fn try_inputs(noun: isize, verb: isize, codes: impl Into<Vec<isize>>) -> isize {
    let mut codes = codes.into();
    codes[1] = noun;
    codes[2] = verb;
    run_program_no_in(&mut codes);
    codes[0]
}

pub fn part2(input: &str) -> isize {
    use itertools::iproduct;
    let desired_output = 19690720;
    let codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    for (noun, verb) in iproduct!((0..=99), (0..=99)) {        
        let result = try_inputs(noun, verb, codes.clone());
        if result == desired_output {                       
            return 100 * noun + verb
        }
    }
    unreachable!("It's assumed the puzzle will have _a_ valid solution");    
}
