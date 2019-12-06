use std::io::{sink, Write};

pub fn run_program_no_in(codes: &mut [isize]) {
    // setting up an empty input as this function assumes programs witn no
    // input (eg for day 2, mostly)
    let input = std::io::empty();
    run_program(codes, input, &mut sink());
}

pub fn run_program(codes: &mut [isize], mut input: impl IntcodeInput, output: &mut impl Write) {
    let mut machine = IntcodeMachine::new(codes);
    while !machine.stopped {
        machine.step(&mut input, output).unwrap();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Opcode {
    pub fn num_params(&self) -> usize {
        use Opcode::*;
        match *self {
            Add | Mul | LessThan | Equals => 3,
            JumpIfTrue | JumpIfFalse => 2,
            Input | Output => 1,
            Halt => 0,
        }
    }
    pub fn cursor_change(&self) -> usize {
        use Opcode::*;
        match *self {
            JumpIfTrue | JumpIfFalse => 0,
            _ => 1 + self.num_params(),
        }
    }
}

const DIGIT_OFFSETS: &'static [isize] = &[1, 10, 100, 1000, 10000];
fn get_digit(num: isize, i: usize) -> isize {
    (num / DIGIT_OFFSETS[i]) % 10
}

impl From<isize> for Opcode {
    fn from(code: isize) -> Self {
        use Opcode::*;
        // the actual instruction is only the last two digits
        match code % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Halt,
            // this should never happen in a well-formed program
            // as the program cursor will only ever really go over opcode positions
            x => panic!("Invalid opcode {}", x),
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Parameter(ParameterMode, isize);

impl Parameter {
    fn immediate(i: isize) -> Self {
        Parameter(ParameterMode::Immediate, i)
    }
    /// Returns just the value indicated by this parameter
    /// according to its mode
    fn find(&self, codes: &[isize]) -> isize {
        match self.0 {
            ParameterMode::Position => codes[self.1 as usize],
            ParameterMode::Immediate => self.1,
        }
    }
    fn find_mut<'a>(&self, codes: &'a mut [isize]) -> &'a mut isize {
        match self.0 {
            ParameterMode::Position => &mut codes[self.1 as usize],
            ParameterMode::Immediate => unreachable!("Told to write with immediate mode"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    params: Vec<Parameter>,
}

impl Instruction {
    fn create(cursor: usize, codes: &[isize]) -> Instruction {
        let instruction = codes[cursor];
        let opcode: Opcode = instruction.into();
        let mut p_modes = (2..=4).map(|i| match get_digit(instruction, i) {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            x => {
                unreachable!("Invalid parameter mode {}", x);
            }
        });
        let mut params = Vec::new();
        for i in 1..=opcode.num_params() {
            params.push(Parameter(p_modes.next().unwrap(), codes[cursor + i]));
        }
        Instruction { opcode, params }
    }

    fn execute<T, W>(
        &mut self,
        cursor: &mut usize,
        codes: &mut [isize],
        input: &mut T,
        out_stream: &mut W,
    ) where
        T: IntcodeInput,
        W: Write,
    {
        let f = match self.opcode {
            Opcode::Input => {
                writeln!(out_stream, "INPUT: ").unwrap();
                let input: &str = &input.read_line().expect("Failed receiving input");
                // little hack: adding the input as a pseudo-parameter of
                // the instruction
                let in_value = input.trim().parse().unwrap();
                self.params.push(Parameter::immediate(in_value));
                ops::place
            }
            Opcode::Output => {
                ops::out(out_stream, &self.params, codes);
                *cursor += Opcode::Output.cursor_change();
                return;
            }
            Opcode::Add => ops::add,
            Opcode::Mul => ops::mul,
            Opcode::JumpIfTrue => ops::jit,
            Opcode::JumpIfFalse => ops::jif,
            Opcode::LessThan => ops::lt,
            Opcode::Equals => ops::eq,
            Opcode::Halt => {
                unreachable!("Should be impossible: Halt is checked before entering this function")
            }
        };
        f(&self.params, codes, cursor);
        *cursor += self.opcode.cursor_change();
    }
}

struct IntcodeMachine<'a> {
    stopped: bool,
    cursor: usize,
    codes: &'a mut [isize],
}

impl<'a> IntcodeMachine<'a> {
    fn new(codes: &'a mut [isize]) -> Self {
        Self {
            stopped: false,
            cursor: 0,
            codes,
        }
    }

    fn step<T>(&mut self, input: &mut T, output: &mut impl Write) -> Result<(), String>
    where
        T: IntcodeInput,
    {
        let mut instruction = Instruction::create(self.cursor, self.codes);
        //println!("cursor: {}, {:?}", self.cursor, instruction);
        if self.stopped {
            return Err("Program Halted".into());
        }
        match instruction.opcode {
            Opcode::Halt => {
                self.stopped = true;
                return Ok(());
            }
            _ => instruction.execute(&mut self.cursor, self.codes, input, output),
        };
        Ok(())
    }
}

pub trait IntcodeInput {
    fn read_line(&mut self) -> std::io::Result<String>;
}

impl IntcodeInput for std::io::Empty {
    fn read_line(&mut self) -> std::io::Result<String> {
        Ok(String::new())
    }
}
impl IntcodeInput for std::io::Lines<std::io::StdinLock<'_>> {
    fn read_line(&mut self) -> std::io::Result<String> {
        self.next().unwrap()
    }
}

impl IntcodeInput for &str {
    fn read_line(&mut self) -> std::io::Result<String> {
        Ok(self.lines().next().unwrap().to_owned())
    }
}

mod ops {
    use super::*;

    fn op_and_place(params: &[Parameter], codes: &mut [isize], f: impl Fn(isize, isize) -> isize) {
        let (x, y, dest) = (
            params[0].find(codes),
            params[1].find(codes),
            params[2].find_mut(codes),
        );
        *dest = f(x, y)
    }
    pub(super) fn add(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
        op_and_place(params, codes, std::ops::Add::add)
    }
    pub(super) fn mul(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
        op_and_place(params, codes, std::ops::Mul::mul)
    }
    pub(super) fn place(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
        codes[params[0].1 as usize] = params[1].1;
    }
    fn jump_if(
        params: &[Parameter],
        codes: &mut [isize],
        cursor: &mut usize,
        cond: impl Fn(isize) -> bool,
    ) {
        if cond(params[0].find(codes)) {
            *cursor = params[1].find(codes) as usize;
        } else {
            *cursor += 3
        }
    }
    pub(super) fn jif(params: &[Parameter], codes: &mut [isize], cursor: &mut usize) {
        jump_if(params, codes, cursor, |v| v == 0)
    }
    pub(super) fn jit(params: &[Parameter], codes: &mut [isize], cursor: &mut usize) {
        jump_if(params, codes, cursor, |v| v != 0)
    }
    pub(super) fn lt(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
        let comp = |a, b| if a < b { 1 } else { 0 };
        op_and_place(params, codes, comp);
    }
    pub(super) fn eq(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
        let comp = |a, b| if a == b { 1 } else { 0 };
        op_and_place(params, codes, comp);
    }

    // out is a special case
    pub(super) fn out(out_stream: &mut impl Write, params: &[Parameter], codes: &mut [isize]) {
        match writeln!(out_stream, "OUT: {}", params[0].find(codes)) {
            _ => (),
        }
    }
}
