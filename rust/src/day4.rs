use std::cmp::Ordering;

const DIGIT_OFFSETS: &'static [u32] = &[1, 10, 100, 1000, 10000, 100000];

fn get_digit(num: u32, digit: usize) -> u32 {
    (num / DIGIT_OFFSETS[digit]) % 10
}

fn valid_password(num: u32) -> (bool, bool) {
    let mut digits = (0..=5).rev().map(|i| get_digit(num, i));
    let mut prev = digits.next().unwrap();
    let mut counts = vec![1];
    for curr in digits {
        match prev.cmp(&curr) {
            Ordering::Greater => return (false, false),
            Ordering::Equal => *counts.last_mut().unwrap() += 1,
            Ordering::Less => counts.push(1),
        }
        prev = curr
    }
    counts.into_iter().fold((false, false), |(v_p1, v_p2), x| {
        ((v_p1 || x >= 2), (v_p2 || x == 2))
    })        
}

pub fn part1_part2(input: &str) -> (u32, u32) {
    let (mut p1_count, mut p2_count) = (0,0);
    let mut input = input.trim().split("-").map(|i| i.parse().unwrap());
    let (low, high) = (input.next().unwrap(), input.next().unwrap());
    for value in low..=high {
        let (p1, p2) = valid_password(value);
        if p1 {
            p1_count += 1
        }
        if p2 {
            p2_count += 1
        }
    }
    
    (p1_count,p2_count)
}
