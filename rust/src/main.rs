use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/08-1.txt");
    let result1= day8::part1(input);
    print!("part 1 {} ", result1);
    println!();
    day8::part2(input);
    //let result2= day8::part2(input);    
    //print!("part 2 {} ", result2);
    println!("");
}
