use super::intcode;

pub fn part1(input: &str) -> isize {
    let mut codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //let stdin = std::io::stdin();
    //let input = stdin.lock().lines();    
    intcode::run_program(&mut codes, "1", &mut std::io::sink());
    codes[0]
}

pub fn part2(input: &str) -> isize {
    let mut codes: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //let stdin = std::io::stdin();
    //let input = stdin.lock().lines();
    intcode::run_program(&mut codes,"5", &mut std::io::sink());
    codes[0]
}
