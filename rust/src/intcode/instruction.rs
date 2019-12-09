use super::{IntcodeInput, IntcodeOutput, Memory, Opcode, Parameter, ParameterMode};

const DIGIT_OFFSETS: &'static [i64] = &[1, 10, 100, 1000, 10000];
fn get_digit(num: i64, i: usize) -> i64 {
    (num / DIGIT_OFFSETS[i]) % 10
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub params: Vec<Parameter>,
}

impl Instruction {
    pub(super) fn create(cursor: usize, memory: &mut Memory) -> Instruction {
        let instruction = memory[cursor];
        let opcode: Opcode = instruction.into();
        let mut p_modes = (2..=4).map(|i| match get_digit(instruction, i) {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            x => {
                unreachable!("Invalid parameter mode {}", x);
            }
        });
        // there are only up to 3 parameters to an instruction
        let mut params = Vec::with_capacity(3);
        for i in 1..=opcode.num_params() {
            params.push(Parameter(p_modes.next().unwrap(), memory[cursor + i]));
        }
        Instruction { opcode, params }
    }

    pub(super) fn execute<I, O>(
        &self,
        cursor: &mut usize,
        memory: &mut Memory,
        input: &mut I,
        output: &mut O,
    ) -> Result<(), String>
    where
        I: IntcodeInput,
        O: IntcodeOutput,
    {
        let f = match self.opcode {
            Opcode::Input => {
                let input: &str = &match input.read_line() {
                    Some(s) => s,
                    None => return Err("Out of Input".to_owned()),
                };
                let in_value = input.trim().parse().expect("failed parse input");
                let dest = &self.params[0];
                *dest.find_mut(memory) = in_value;
                return Ok(());
            }
            Opcode::Output => {
                output.write(self.params[0].find(memory));
                return Ok(());
            }
            Opcode::Halt => {
                unreachable!("Should be impossible: Halt is checked before this function")
            }
            Opcode::Add => ops::add,
            Opcode::Mul => ops::mul,
            Opcode::JumpIfTrue => ops::jit,
            Opcode::JumpIfFalse => ops::jif,
            Opcode::LessThan => ops::lt,
            Opcode::Equals => ops::eq,
            Opcode::MoveRelative => ops::mov_rel,
        };
        f(&self.params, memory, cursor);
        Ok(())
    }
}

pub(super) mod ops {
    use super::*;
    fn op_and_place(params: &[Parameter], mem: &mut Memory, f: impl Fn(i64, i64) -> i64) {
        let (x, y, dest) = (
            params[0].find(mem),
            params[1].find(mem),
            // third param will be written to:
            params[2].find_mut(mem),
        );
        *dest = f(x, y);
    }

    pub(super) fn add(params: &[Parameter], mem: &mut Memory, _cursor: &mut usize) {
        op_and_place(params, mem, std::ops::Add::add)
    }
    pub(super) fn mul(params: &[Parameter], mem: &mut Memory, _cursor: &mut usize) {
        op_and_place(params, mem, std::ops::Mul::mul)
    }

    fn jump_if(
        params: &[Parameter],
        mem: &mut Memory,
        cursor: &mut usize,
        cond: impl Fn(i64) -> bool,
    ) {
        if cond(params[0].find(mem)) {
            *cursor = params[1].find(mem) as usize;
        } else {
            *cursor += 3
        }
    }

    pub(super) fn jif(params: &[Parameter], mem: &mut Memory, cursor: &mut usize) {
        jump_if(params, mem, cursor, |v| v == 0)
    }
    pub(super) fn jit(params: &[Parameter], mem: &mut Memory, cursor: &mut usize) {
        jump_if(params, mem, cursor, |v| v != 0)
    }

    pub(super) fn lt(params: &[Parameter], mem: &mut Memory, _: &mut usize) {
        let comp = |a, b| if a < b { 1 } else { 0 };
        op_and_place(params, mem, comp);
    }
    pub(super) fn eq(params: &[Parameter], mem: &mut Memory, _: &mut usize) {
        let comp = |a, b| if a == b { 1 } else { 0 };
        op_and_place(params, mem, comp);
    }

    pub(super) fn mov_rel(params: &[Parameter], mem: &mut Memory, _: &mut usize) {
        mem.relative_base += params[0].find(mem);
    }
}
