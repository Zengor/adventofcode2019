use std::io::{sink, Write};

pub fn run_program_no_in(codes: &[isize]) {
    // setting up an empty input as this function assumes programs witn no
    // input (eg for day 2, mostly)
    let mut input = std::io::empty();
    run_program(codes, &mut input, &mut sink());
}

pub fn run_program<I, O>(codes: &[isize], input: &mut I, output: &mut O)
where
    I: IntcodeInput,
    O: IntcodeOutput,
{
    let mut machine = IntcodeMachine::copy_program(codes);
    machine.run_until_halt(input, output)
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

    fn execute<T, O>(
        &mut self,
        cursor: &mut usize,
        codes: &mut [isize],
        input: &mut T,
        out_stream: &mut O,
    ) -> Result<(), String>
    where
        T: IntcodeInput,
        O: IntcodeOutput,
    {
        let f = match self.opcode {
            Opcode::Input => {
                let input: &str = &match input.read_line() {
                    Some(s) => s,
                    None => return Err("Out of Input".to_owned())
                };
                // little hack: adding the input as a pseudo-parameter of
                // the instruction
                let in_value = input.trim().parse().unwrap();
                self.params.push(Parameter::immediate(in_value));
                ops::place
            }
            Opcode::Output => {
                ops::out(out_stream, &self.params, codes);
                *cursor += Opcode::Output.cursor_change();
                return Ok(());
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
        Ok(())
    }
}

pub struct IntcodeMachine {
    stopped: bool,
    cursor: usize,
    codes: Vec<isize>,
}

impl<'a> IntcodeMachine {
    pub fn copy_program(codes: &[isize]) -> Self {
        Self {
            stopped: false,
            cursor: 0,
            codes: codes.into(),
        }
    }

    pub fn is_stopped(&self) -> bool { self.stopped }

    pub fn reset(&mut self) {
        self.stopped = true;
        self.cursor = 0;
    }
    
    pub fn run_until_halt<I, O>(&mut self, input: &mut I, output: &mut O)
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            self.step(input, output).unwrap();
        }
    }
    // runs with given input until it's empty
    // (runs indefinitely with sink
    pub fn run_while_input<I, O>(&mut self, input: &mut I, output: &mut O)
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            match self.step(input, output) {
                Ok(()) => (),
                Err(_) => {
                    return;
                }
            };
        }
    }

    pub fn step<I, O>(&mut self, input: &mut I, output: &mut O) -> Result<(), String>
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        let mut instruction = Instruction::create(self.cursor, &self.codes);
        // println!("cursor: {}, {:?}", self.cursor, instruction);
        if self.stopped {
            return Err("Program Halted".into());
        }
        match instruction.opcode {
            Opcode::Halt => {
                self.stopped = true;
                return Ok(());
            }
            _ => instruction.execute(&mut self.cursor, &mut self.codes, input, output)?,
        };
        Ok(())
    }
}

pub trait IntcodeInput {
    fn read_line(&mut self) -> Option<String>;
}

impl IntcodeInput for std::io::Empty {
    fn read_line(&mut self) -> Option<String> {
        Some(String::new())
    }
}
impl IntcodeInput for std::io::Lines<std::io::StdinLock<'_>> {
    fn read_line(&mut self) -> Option<String> {
        self.next()?.ok()
    }
}

impl IntcodeInput for &str {
    fn read_line(&mut self) -> Option<String> {
        Some(self.lines().next()?.to_owned())
    }
}

impl IntcodeInput for Vec<isize> {
    fn read_line(&mut self) -> Option<String> {
        if self.len() == 0 {
            return None;
        }
        Some(self.remove(0).to_string())
    }
}

impl IntcodeInput for Option<isize> {
    fn read_line(&mut self) -> Option<String> {
        self.take().map(|i| i.to_string())
    }
}


pub trait IntcodeOutput {
    fn write(&mut self, out: isize);
}

impl IntcodeOutput for std::io::Sink {
    fn write(&mut self, _: isize) {}
}
impl IntcodeOutput for std::io::StdoutLock<'_> {
    fn write(&mut self, out: isize) {
        match writeln!(self, "OUTPUT: {}", out) {
            // silently fail
            _ => (),
        };
    }
}
impl IntcodeOutput for Vec<isize> {
    fn write(&mut self, out: isize) {
        self.push(out)
    }
}

impl IntcodeOutput for Option<isize> {
    fn write(&mut self, out: isize) {
        *self = Some(out);
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
    pub(super) fn out(
        out_stream: &mut impl IntcodeOutput,
        params: &[Parameter],
        codes: &mut [isize],
    ) {
        out_stream.write(params[0].find(codes))
    }
}
