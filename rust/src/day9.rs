use crate::intcode::run_from_str;

pub fn part1(input: &str) -> i64 {
    let mut out = None;
    run_from_str(input, &mut Some(1), &mut out);
    out.unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut out = None;
    run_from_str(input, &mut Some(2), &mut out);
    out.unwrap()
}
