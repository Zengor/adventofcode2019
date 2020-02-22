//! # Day 4: Secure Container
//!
//! ## Problem Description
//!
//! Given a range of numbers and some rules, check how many numbers in
//! that range fit the rules. Part 2 has a slight variation on one of
//! the rules, but is otherwise the same. For both parts, the digits
//! may never decrease from left to right. For Part 1, there must be
//! some sequence of repeating digits. For Part 2 there must be a
//! sequence of _exactly_ two repeating digits.
//!
//! ## Implementention details
//!
//! Unlike most other days, I just did parts 1 and 2 as the same
//! function. For each number, I keep a Vec to store the size of
//! sequences of digits, and iterate through its digits
//! once. Immediately return false if there is a decrease. If the
//! current digit is equal to the previous, increase the value of the
//! last count in the Vec. If the current is greater than the
//! previous, start counting a new sequence by pushing 1 into the
//! Vec. After iterating, just need to go through the counts Vec and
//! check password validity for both parts.

use itertools::Itertools;
use std::cmp::Ordering;

const DIGIT_OFFSETS: &'static [u32] = &[1, 10, 100, 1000, 10000, 100000];

fn get_digits(num: u32) -> impl Iterator<Item = u32> {
    (0..5).rev().map(move |i| (num / DIGIT_OFFSETS[i]) % 10)
}

fn valid_password(num: u32) -> (bool, bool) {
    let mut digits = get_digits(num);
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
    let (mut p1_count, mut p2_count) = (0, 0);
    let (low, high) = input
        .trim()
        .split("-")
        .map(|i| i.parse().unwrap())
        .collect_tuple()
        .unwrap();
    for value in low..=high {
        let (p1, p2) = valid_password(value);
        if p1 {
            p1_count += 1
        }
        if p2 {
            p2_count += 1
        }
    }
    (p1_count, p2_count)
}
