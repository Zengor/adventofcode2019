use super::intcode;

pub fn part1(input: &str) -> isize {
    let mut codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    intcode::run_program(&mut codes);
    codes[0]
}
