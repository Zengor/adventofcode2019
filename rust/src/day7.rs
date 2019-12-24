//! Day 7: Amplification Circuit
//!
//! # Problem Description
//!
//! Another Intcode problem. This time you have to run 5 separate
//! instances of the same program, where the output of one machine
//! feeds into the next one. The idea is that the first input (called
//! the "phase setting") of each machine is a number between 0 and 4
//! given at the start of execution, and you have to find the
//! combination which results into the highest total value at the end,
//! where no phase setting may be shared between two machines.
//!
//! In part 1, machines only pass each other input once. In part
//!
//! #Implementation Details
//!
//!

use super::intcode::IntcodeMachine;

use std::ops::Range;

use itertools::Itertools;

fn find_max_thruster(input: &str, setting_range: Range<i64>, loop_limit: usize) -> i64 {
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().expect("failedparse"))
        .collect();
    let permutations = setting_range.permutations(5).filter(|permutation| {
        itertools::all(permutation.iter().tuple_combinations(), |(a, b)| a != b)
    });
    permutations
        .map(|ps| amp_exec(&codes, ps, loop_limit))
        .max()
        .unwrap()
}

fn amp_exec(codes: &[i64], phases: Vec<i64>, loop_limit: usize) -> i64 {
    let len = phases.len();
    let mut phases = phases.into_iter();
    let mut machines: Vec<_> = (0..len)
        .map(|_| {
            let mut m = IntcodeMachine::copy_program(&codes);
            m.step(&mut phases.next(), &mut std::io::sink());
            m
        })
        .collect();
    let mut input = Some(0i64);
    let mut out = None;
    for i in 0.. {
        machines[i % len].run_while_input(&mut input, &mut out);
        if i / len >= loop_limit || itertools::all(&machines, |m| m.is_stopped()) {
            break;
        }
        input = out;
    }

    out.unwrap()
}

pub fn part1(input: &str) -> i64 {
    find_max_thruster(input, 0..5, 1)
}

pub fn part2(input: &str) -> i64 {
    find_max_thruster(input, 5..10, std::usize::MAX)
}
