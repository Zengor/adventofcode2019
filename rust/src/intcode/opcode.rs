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
