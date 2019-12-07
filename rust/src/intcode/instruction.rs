use super::{IntcodeInput, IntcodeOutput, Opcode, Parameter, ParameterMode};

const DIGIT_OFFSETS: &'static [isize] = &[1, 10, 100, 1000, 10000];
fn get_digit(num: isize, i: usize) -> isize {
    (num / DIGIT_OFFSETS[i]) % 10
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub params: Vec<Parameter>,
}

impl Instruction {
    pub fn create(cursor: usize, codes: &[isize]) -> Instruction {
        let instruction = codes[cursor];
        let opcode: Opcode = instruction.into();
        let mut p_modes = (2..=4).map(|i| match get_digit(instruction, i) {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            x => {
                unreachable!("Invalid parameter mode {}", x);
            }
        });
        // there are only up to 3 parameters to an instruction
        // and it's not a heavy memory cost to preallocate for all of them
        let mut params = Vec::with_capacity(3);
        for i in 1..=opcode.num_params() {
            params.push(Parameter(p_modes.next().unwrap(), codes[cursor + i]));
        }
        Instruction { opcode, params }
    }

    pub fn execute<T, O>(
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
                    None => return Err("Out of Input".to_owned()),
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
