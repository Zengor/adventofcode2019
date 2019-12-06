use adventofcode2019::*;

fn main() {
    let input = include_str!("../../input/06-1.txt");
    //let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    let result1= day6::part1(input);
    let result2= day6::part2(input);
    //let result2 = 0;
    println!("part 1 {} part 2 {}", result1, result2);
}
