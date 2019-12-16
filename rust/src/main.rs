use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/16-1.txt");
    //let input = "12345678";
    let result1= day16::part1(input);
    print!("part 1 {:?} ", &result1[0..8]);
    
    println!();
    //let input = "000000712345678";
    //let input = "03036732577212944063491565474664";
    //let input = "03081770884921959731165446850517";
    let result2= day16::part2(input);    
    print!("part 2 {} ", result2);
    println!("");
}
