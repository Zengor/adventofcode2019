use std::io::sink;

mod instruction;
mod io;
mod opcode;

pub use opcode::Opcode;
pub use instruction::Instruction;
pub use io::{IntcodeInput, IntcodeOutput};

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

#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
pub struct Parameter(ParameterMode, isize);

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

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

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
