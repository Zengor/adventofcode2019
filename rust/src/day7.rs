use super::intcode::{self, IntcodeMachine};

use std::ops::Range;

use itertools::Itertools;

fn find_max_thruster(
    input: &str,
    setting_range: Range<isize>,
    amp_control: impl Fn(&[isize], Vec<isize>) -> isize,
) -> isize {
    let codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().expect("failedparse"))
        .collect();
    setting_range
        .permutations(5)
        .filter(|permutation| {
            itertools::all(permutation.iter().tuple_combinations(), |(a, b)| a != b)
        })
        .map(|ps| amp_control(&codes, ps))
        .max()
        .unwrap()
}

fn amp_chain_once(codes: &[isize], phases: Vec<isize>) -> isize {
    let mut out = Some(0);
    for phase in phases {
        let additional_input = out.take().unwrap();
        let mut actual_input = vec![phase, additional_input];
        intcode::run_program(codes, &mut actual_input, &mut out);
    }
    out.unwrap()
}

fn amp_feedback_loop(codes: &[isize], phases: Vec<isize>) -> isize {
    // machine loops
    let len = phases.len();
    let mut phases = phases.into_iter();
    let mut machines: Vec<_> = (0..len)
        .map(|_| {
            let mut m = IntcodeMachine::copy_program(&codes);
            m.step(&mut phases.next(), &mut std::io::sink()).unwrap();
            m
        })
        .collect();
    let mut input = Some(0);
    let mut out = None;
    for i in 0.. {
        machines[i % len].run_while_input(&mut input, &mut out);
        if itertools::all(&machines, |m| m.is_stopped()) {
            break;
        }
        input = out;
    }
    out.unwrap()
}

pub fn part1(input: &str) -> isize {
    find_max_thruster(input, 0..5, amp_chain_once)
}

pub fn part2(input: &str) -> isize {
    find_max_thruster(input, 5..10, amp_feedback_loop)
}
