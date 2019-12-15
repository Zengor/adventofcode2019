use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/14-1.txt");
    let result1= day14::part1(input);
    print!("part 1 {} ", result1);
    println!();
    let result2= day14::part2(input);    
    print!("part 2 {} ", result2);
    println!("");
}
