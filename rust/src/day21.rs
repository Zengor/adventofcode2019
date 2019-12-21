use crate::intcode::run_program_from_str;

const PART1: &str =
    "OR A J
AND B J
AND C J
NOT J J
AND D J
WALK\n";
const PART2: &str = "OR A T
AND B T
AND C T
NOT T T
OR E J
OR H J
AND T J
AND D J
RUN\n";
    
pub fn part1(input: &str) -> i64 {
    let mut solution: Vec<_> = PART1.chars().map(|c| c as i64).collect();
    let mut out = None;
    run_program_from_str(input, &mut solution, &mut out);
    out.unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut solution: Vec<_> = PART2.chars().map(|c| c as i64).collect();
    let mut out = None;
    run_program_from_str(input, &mut solution, &mut out);
    out.unwrap()
}
