fn transform(x: i32) -> i32 {
    x / 3 - 2
}

pub fn part1(input: &str) -> i32 {      
    input
        .lines()
        .map(|l| l.parse().expect("failed parsing line"))
        .map(transform)
        .sum()
}


fn continuous_transform(mut x: i32) -> i32 {
    let mut sum = 0;
    x = transform(x);
    while x > 0 {
        sum += x;
        x = transform(x);
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse().expect("failed parsing line"))
        .map(continuous_transform)
        .sum()
}
