//! # Day 1: The Tyranny of the Rocket Equation
//!
//! ## Problem Description
//!
//! Given a list of weights, find the weight fuel required by all the
//! weights using the calculation f(x) = (x / 3) - 2.  In part 1, this
//! function is applied once, in part 2 it's applied repeatedly for
//! the fuel itself until the result is 0 or less, that is `g(x) where
//! x < 0 = 0;` `g(x) = f(x) + g(f(x))`
//!
//! ## Implementention details
//!
//! Just map a funcion over all lines and call `sum()`. `total_fuel`
//! was implemented iteratively, not recursively. Nothing to really
//! say beyond that.

pub fn fuel_needed(x: i32) -> i32 {
    x / 3 - 2
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse().expect("failed parsing line"))
        .map(fuel_needed)
        .sum()
}

pub fn total_fuel(mut x: i32) -> i32 {
    let mut sum = 0;
    x = fuel_needed(x);
    while x > 0 {
        sum += x;
        x = fuel_needed(x);
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse().expect("failed parsing line"))
        .map(total_fuel)
        .sum()
}
