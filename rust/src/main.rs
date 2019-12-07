use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/07-1.txt");
    let result1= day7::part1(input);
    print!("part 1 {} ", result1);
    let result2= day7::part2(input);    
    print!("part 2 {} ", result2);
    println!("");
}
