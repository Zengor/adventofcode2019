use itertools::Itertools;

use crate::intcode;

pub fn part1(input: &str) -> usize {
    let mut output: Vec<i64> = Vec::new();
    intcode::run_program_from_str(input, &mut std::io::empty(), &mut output);
    output.into_iter().tuples().filter(|(_,_,t)|  *t == 2).count() 
}
