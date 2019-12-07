use super::intcode::{self, IntcodeMachine};

use std::ops::Range;

use itertools::Itertools;

fn find_max_thruster(input: &str, setting_range: Range<isize>, loop_limit: usize) -> isize {
    let codes: Vec<isize> = input
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

fn amp_exec(codes: &[isize], phases: Vec<isize>, loop_limit: usize) -> isize {
    let len = phases.len();
    let mut phases = phases.into_iter();
    let mut machines: Vec<_> = (0..len)
        .map(|_| {
            let mut m = IntcodeMachine::copy_program(&codes);
            m.step(&mut phases.next(), &mut std::io::sink()).unwrap();
            m
        })
        .collect();
    let mut input = Some(0isize);
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

pub fn part1(input: &str) -> isize {
    find_max_thruster(input, 0..5, 1)
}

pub fn part2(input: &str) -> isize {
    find_max_thruster(input, 5..10, std::usize::MAX)
}
