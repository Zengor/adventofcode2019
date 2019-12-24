use std::io::sink;

mod instruction;
mod io;
mod opcode;

pub use instruction::Instruction;
pub use io::{IntcodeInput, IntcodeOutput};
pub use opcode::Opcode;

/// Convenience function for early days to just run a program with no
/// I/O, returning the value at memory position 0 at the end.
pub fn run_program_no_io(codes: &[i64]) -> i64 {
    let input = &mut std::io::empty();
    let output = &mut std::io::sink();
    
    let mut machine = IntcodeMachine::copy_program(codes);
    machine.run_no_io();
    machine.get(0)
}

pub fn run_from_str<I, O>(codes: &str, input: &mut I, output: &mut O) -> i64
where
    I: IntcodeInput,
    O: IntcodeOutput,
{
    let mut machine = IntcodeMachine::from_str(codes);
    machine.run(input, output);
    machine.get(0)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub struct Parameter(ParameterMode, i64);

impl Parameter {
    fn find(&self, memory: &mut Memory) -> i64 {
        use ParameterMode::*;
        if self.0 == Immediate {
            return self.1;
        };
        let pos = match self.0 {
            Position => self.1,
            Relative => memory.relative_base + self.1,
            Immediate => unreachable!(),
        };
        memory.get(pos)
    }
    fn find_mut<'a>(&self, memory: &'a mut Memory) -> &'a mut i64 {
        use ParameterMode::*;
        if self.0 == Immediate {
            panic!("Attempted to write using immediate mode");
        };
        let pos = match self.0 {
            Position => self.1,
            Relative => memory.relative_base + self.1,
            Immediate => unreachable!(),
        };
        memory.get_mut(pos)
    }
}

pub enum RunResult {
    Stop,
    Continue,
    InputRequest,
    Output,
}

#[derive(Debug, Clone)]
pub struct IntcodeMachine {
    stopped: bool,
    cursor: usize,
    mem: Memory,
}

impl<'a> IntcodeMachine {
    pub fn copy_program(codes: &[i64]) -> Self {
        let mem = Memory::with(codes);
        Self {
            stopped: false,
            cursor: 0,
            mem,
        }
    }

    pub fn from_str(input: &str) -> Self {
        let mem = Memory::from_str(input);
        Self {
            stopped: false,
            cursor: 0,
            mem,
        }
    }

    pub fn get(&self, i: usize) -> i64 {
        self.mem.mem[i]
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn reset(&mut self) {
        self.stopped = true;
        self.cursor = 0;
    }

    pub fn run_no_io(&mut self) -> RunResult {
        self.run(&mut std::io::empty(), &mut std::io::sink())
    }

    pub fn run<I, O>(&mut self, input: &mut I, output: &mut O) -> RunResult
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            match self.step(input, output) {
                r @ RunResult::InputRequest => return r,
                _ => (),
            }
        }
        RunResult::Stop
    }
    
    /// Runs the machine with specified input. When the input is empty _and_ a instruction
    /// requires additional input, it will stop. Otherwise, runs until end
    pub fn run_while_input<I, O>(&mut self, input: &mut I, output: &mut O) -> RunResult
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            match self.step(input, output) {
                r @ RunResult::InputRequest => return r,
                _ => (),
            }
        }
        RunResult::Stop
    }

    /// Runs the machine with specified input, stopping after the
    /// first input instruction. This doesn't assume the very next
    /// instruction will be an input instruction, running normally
    /// until input is consumed exactly once. However, the program may
    /// theoretically still halt. This is intended mostly to be a
    /// convenience way to queue some initializer input to the machine.
    pub fn run_single_input<I, O>(&mut self, input: &mut I, output: &mut O) -> RunResult
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            // a little bit of a hack: check if current instruction
            // will be an input instruction
            let is_input = self.mem.mem[self.cursor] == 3;
            self.step(input, output);
            if is_input {
                return RunResult::Continue;
            }
        }
        RunResult::Stop
    }

    pub fn step<I, O>(&mut self, input: &mut I, output: &mut O) -> RunResult
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        if self.stopped {
            return RunResult::Stop;
        }
        let instruction = Instruction::create(self.cursor, &mut self.mem);
        match instruction.opcode {
            Opcode::Halt => {
                self.stopped = true;
                return RunResult::Stop;
            }
            Opcode::Input => {
                let input = match input.read() {
                    Some(i) => i,
                    None => return RunResult::InputRequest,
                };
                let dest = &instruction.params[0];
                *dest.find_mut(&mut self.mem) = input;
            }
            Opcode::Output => {
                let out = instruction.params[0].find(&mut self.mem);
                self.cursor += instruction.opcode.cursor_change();
                output.write(out);
                return RunResult::Output;
            }
            _ => {
                instruction.execute(&mut self.cursor, &mut self.mem);
            }
        }

        self.cursor += instruction.opcode.cursor_change();
        return RunResult::Continue;
    }
}

#[derive(Clone, Debug)]
struct Memory {
    pub relative_base: i64,
    mem: Vec<i64>,
}

impl Memory {
    pub fn with_capacity(capacity: usize) -> Self {
        let mem = Vec::with_capacity(capacity);
        Self {
            relative_base: 0,
            mem,
        }
    }
    pub fn with(codes: &[i64]) -> Self {
        let mut mem = Self::with_capacity(codes.len() + 3000);
        mem.mem.extend_from_slice(codes);
        mem.reserve_up_to(codes.len() + 3000);
        mem
    }
    pub fn from_str(input: &str) -> Self {
        let mut mem: Vec<i64> = input
            .trim()
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();
        mem.resize(mem.len() + 2000, 0);
        Self {
            relative_base: 0,
            mem,
        }
    }

    fn reserve_up_to(&mut self, pos: usize) {
        if pos >= self.mem.len() {
            let new_len = self.mem.len() + (pos + 1 - self.mem.len());
            self.mem.resize(new_len, 0);
        }
    }

    fn get(&mut self, pos: i64) -> i64 {
        let pos = pos as usize;
        self.reserve_up_to(pos);
        self.mem[pos]
    }

    fn get_mut(&mut self, pos: i64) -> &mut i64 {
        let pos = pos as usize;
        self.reserve_up_to(pos);
        &mut self.mem[pos]
    }
}

impl std::ops::Index<usize> for Memory {
    type Output = i64;
    fn index(&self, idx: usize) -> &i64 {
        &self.mem[idx]
    }
}
impl std::ops::Index<i64> for Memory {
    type Output = i64;
    fn index(&self, idx: i64) -> &i64 {
        &self.mem[idx as usize]
    }
}
