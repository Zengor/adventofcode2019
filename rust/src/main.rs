use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/09-1.txt");
    let result1= day9::part1(input);
    print!("part 1 {} ", result1);
    println!();
    let result2= day9::part2(input);    
    print!("part 2 {} ", result2);
    println!("");
}
