use std::io::sink;

mod instruction;
mod io;
mod opcode;

pub use instruction::Instruction;
pub use io::{IntcodeInput, IntcodeOutput};
pub use opcode::Opcode;

pub fn run_program_no_in(codes: &[i64]) -> i64{
    // setting up an empty input as this function assumes programs witn no
    // input (eg for day 2, mostly)
    let mut input = std::io::empty();
    run_program(codes, &mut input, &mut sink())
}

pub fn run_program<I, O>(codes: &[i64], input: &mut I, output: &mut O) -> i64
where
    I: IntcodeInput,
    O: IntcodeOutput,
{
    let mut machine = IntcodeMachine::copy_program(codes);
    machine.run_until_halt(input, output)
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
            Immediate => unreachable!()
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
            Immediate => unreachable!()
        };
        memory.get_mut(pos)
    }
}

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

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn reset(&mut self) {
        self.stopped = true;
        self.cursor = 0;
    }

    pub fn run_until_halt<I, O>(&mut self, input: &mut I, output: &mut O) -> i64
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        while !self.stopped {
            self.step(input, output).unwrap();
        }
        self.mem[0usize]
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
        let instruction = Instruction::create(self.cursor, &mut self.mem);
        // println!("cursor: {}, {:?}", self.cursor, instruction);
        if self.stopped {
            return Err("Program Halted".into());
        }
        if instruction.opcode == Opcode::Halt {
            self.stopped = true;
            return Ok(());
        }
        instruction.execute(&mut self.cursor, &mut self.mem, input, output)?;
        self.cursor += instruction.opcode.cursor_change();
        Ok(())
    }

}


#[derive(Debug)]
struct Memory {
    pub relative_base: i64,
    mem: Vec<i64>
}

impl Memory {
    pub fn with_capacity(capacity: usize) -> Self {
        let mem = Vec::with_capacity(capacity);
         Self {
            relative_base: 0,
            mem
        }
    }
    pub fn with(codes: &[i64]) -> Self {
        let mut mem = Self::with_capacity(codes.len() + 2000);
        mem.mem.extend_from_slice(codes);
        mem.reserve_up_to(codes.len() + 2000);
        mem
    }

    fn reserve_up_to(&mut self, pos: usize) {
        if pos > self.mem.len() {
            let new_len = self.mem.len() + (pos - self.mem.len());
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

