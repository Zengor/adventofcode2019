use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/04-1.txt");
    let (result1, result2) = day4::part1_part2(input);
    println!("part 1 {} part 2 {}", result1, result2);
}
