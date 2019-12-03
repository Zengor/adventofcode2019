#[derive(PartialEq, Eq)]
enum Intcode {
    Add,
    Mul,
    Halt,
    // this should end never being used in a well-formed program
    // as the program cursor will only ever really convert opcode positions
    Value(usize),
}

impl From<usize> for Intcode {
    fn from(code: usize) -> Self {
        use Intcode::*;
        match code {
            1 => Add,
            2 => Mul,
            99 => Halt,
            x => Value(x),
        }
    }
}

fn get_operands(cursor: usize, codes: &[usize]) -> (usize, usize, usize) {
    (codes[cursor + 1], codes[cursor + 2], codes[cursor + 3])
}

fn operate(cursor: usize, function: impl Fn(usize, usize) -> usize, codes: &mut [usize]) {
    let (op1, op2, result_pos) = get_operands(cursor, codes);
    codes[result_pos] = function(codes[op1], codes[op2]);
}

fn run_program(codes: &mut [usize]) {
    let mut cursor = 0;
    let mut next_instruction: Intcode = codes[cursor].into();
    while next_instruction != Intcode::Halt {
        let function = match next_instruction {
            Intcode::Add => std::ops::Add::add,
            Intcode::Mul => std::ops::Mul::mul,
            _ => unreachable!("This shouldn't happen"),
        };
        operate(cursor, function, codes);
        cursor += 4;
        next_instruction = codes[cursor].into();
    }
}

pub fn part1(input: &str) -> usize {
    // as the codes are used for indexing and can never be negative, i use usize
    // instead of a signed integer type
    let mut codes: Vec<usize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    try_inputs(12, 2, codes)
}

pub fn try_inputs(noun: usize, verb: usize, codes: impl Into<Vec<usize>>) -> usize {
    let mut codes = codes.into();
    codes[1] = noun;
    codes[2] = verb;
    run_program(&mut codes);
    codes[0]
}

pub fn part2(input: &str) -> usize {
    use itertools::iproduct;
    let desired_output = 19690720;
    let codes: Vec<usize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    for (noun, verb) in iproduct!((0..=99), (0..=99)) {        
        let result = try_inputs(noun, verb, codes.clone());
        if result == desired_output {                       
            return 100 * noun + verb
        }
    }
    unreachable!("It's assumed the puzzle will have _a_ valid solution");    
}
