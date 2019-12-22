use itertools::join;
use std::iter::repeat;

/// Returns iterator for the pattern of a given digit with position
/// `i`.  Unlike what is described in the problem, this pattern starts
/// with the first 1, and as such is intended for use with the i-th
/// digit of the input.
fn find_pattern(digit: usize) -> impl Iterator<Item = i32> {
    let pattern = repeat(1)
        .take(digit + 1)
        .chain(repeat(0).take(digit + 1))
        .chain(repeat(-1).take(digit + 1))
        .chain(repeat(0).take(digit + 1));
    pattern.cycle()
}

fn fft(mut digits: Vec<i32>, phases: usize) -> Vec<i32> {
    let size = digits.len();
    let mut old_buffer = Vec::with_capacity(size);
    for _phase in 1..=phases {
        old_buffer.clear();
        old_buffer.append(&mut digits);
        digits.extend((0..size).map(|i| {
            find_pattern(i)
                .zip(old_buffer.iter().skip(i))
                .map(|(p, d)| p * d)
                .sum::<i32>()
                .abs()
                % 10
        }));
    }
    digits
}

pub fn part1(input: &str) -> String {
    let digits: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let digits = fft(digits, 100);
    join(&digits[0..8], "")
}

pub fn part2(input: &str) -> String {
    let msg_offset: usize = input[0..7].parse().unwrap();
    let digits: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut digits: Vec<i32> = std::iter::repeat(digits.clone())
        .take(10_000)
        .flatten()
        .skip(msg_offset)
        .collect();
    let size = digits.len();
    for _phase in 0..100 {
        for i in (0..size - 1).rev() {
            digits[i] += digits[i + 1];
        }
        digits.iter_mut().for_each(|x| *x %= 10);
    }
    join(&digits[..8], "")
}
