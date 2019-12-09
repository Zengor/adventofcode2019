use crate::intcode::run_program;

pub fn part1(input: &str) -> i64 {
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().expect("failedparse"))
        .collect();
    let mut out = None;
    run_program(&codes, &mut Some(1), &mut out);
    out.unwrap()
}
pub fn part2(input: &str) -> i64 {
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().expect("failedparse"))
        .collect();
    let mut out = None;
    run_program(&codes, &mut Some(2), &mut out);
    out.unwrap()
}
