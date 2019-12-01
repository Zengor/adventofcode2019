fn fuel_needed(x: i32) -> i32 {
    x / 3 - 2
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse().expect("failed parsing line"))
        .map(fuel_needed)
        .sum()
}

fn total_fuel(mut x: i32) -> i32 {
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
