use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/01-1.txt");
    let result1 = day1::part1(input);
    let result2 = day1::part2(input);
    println!("part 1 {} part 2 {}", result1, result2);
}
