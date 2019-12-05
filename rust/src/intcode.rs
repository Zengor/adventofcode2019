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
            JumpIfTrue |JumpIfFalse => 2,
            Input | Output => 1,
            Halt => 0,
        }
    }
    pub fn cursor_change(&self) -> usize {
        use Opcode::*;
        match *self {
            JumpIfTrue | JumpIfFalse => 0,
            _ => 1 + self.num_params()
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
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl From<isize> for ParameterMode {
    fn from(code: isize) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            x => {
                println!("Invalid parameter mode {}", x);
                // will pick this as default just to theoretically avoid crashing
                ParameterMode::Position
            }
        }
    }
}

#[derive(Debug)]
struct Parameter(ParameterMode, isize);

impl Parameter {
    fn immediate(i: isize) -> Self {
        Parameter(ParameterMode::Immediate, i)
    }
    /// Returns just the value indicated by this parameter
    /// according to is mode
    fn find_value(&self, codes: &[isize]) -> isize {
        match self.0 {
            ParameterMode::Position => codes[self.1 as usize],
            ParameterMode::Immediate => self.1,
        }
    }    
    fn find_value_mut<'a> (&self, codes: &'a mut [isize]) -> &'a mut isize {        
        match self.0 {
            ParameterMode::Position => &mut codes[self.1 as usize],
            ParameterMode::Immediate => unreachable!("Told to write with immediate mode"),
        }
    }
}

pub struct Instruction {
    params: Vec<Parameter>,
    opcode: Opcode,
}

fn create_instruction(cursor: usize, codes: &[isize]) -> Instruction {
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
    Instruction { params, opcode }
}

fn add(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    let result = params[0].find_value(codes) + params[1].find_value(codes);
    *params[2].find_value_mut(codes) = result;
}
fn mul(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    let result = params[0].find_value(codes) * params[1].find_value(codes);
    *params[2].find_value_mut(codes) = result;
}
fn place(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    codes[params[0].1 as usize] = params[1].1;
}
fn out(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    println!("OUT: {}", params[0].find_value(codes));
}

fn jif(params: &[Parameter], codes: &mut [isize], cursor: &mut usize) {
    if params[0].find_value(codes) == 0 {
        *cursor = params[1].find_value(codes) as usize;
    } else {
        // the cursor doesn't automatically increase when this instruction
        // is called, so we have to increase it manually if there is no jump
        *cursor += 3;
    }   
}
fn jit(params: &[Parameter], codes: &mut [isize], cursor: &mut usize) {
    if params[0].find_value(codes) != 0 {
        *cursor = params[1].find_value(codes) as usize;
    } else {
        // the cursor doesn't automatically increase when this instruction
        // is called, so we have to increase it manually if there is no jump
        *cursor += 3;
    }   
}
fn lt(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    let result = if params[0].find_value(codes) < params[1].find_value(codes) {
        1
    } else {
        0
    };
    *params[2].find_value_mut(codes) = result;
}
fn eq(params: &[Parameter], codes: &mut [isize], _: &mut usize) {
    let result = if params[0].find_value(codes) == params[1].find_value(codes) {
        1
    } else {
        0
    };
    *params[2].find_value_mut(codes) = result;
}

pub fn run_program(codes: &mut [isize]) {
    let mut cursor = 0;
    let mut instruction = create_instruction(cursor, codes);
    let mut input = String::new();
    while instruction.opcode != Opcode::Halt {
        //println!("{:?}", codes);
        println!(
             "cursor {}, opcode {:?} params {:?}",
             cursor, instruction.opcode, &instruction.params
        );
        let function = match instruction.opcode {
            Opcode::Add => add,
            Opcode::Mul => mul,
            Opcode::Input => {
                println!("INPUT: ");
                std::io::stdin().read_line(&mut input).expect("Failed reading input");
                // little hack: adding the input as a pseudo-parameter of
                // the instruction
                let in_value = input.trim().parse().unwrap();
                instruction.params.push(Parameter::immediate(in_value));
                place
            }
            Opcode::Output => out,
            Opcode::JumpIfTrue => jit,
            Opcode::JumpIfFalse => jif,
            Opcode::LessThan => lt,
            Opcode::Equals => eq,
            Opcode::Halt => unreachable!("Halt instruction should have stopped loop")
        };
        function(&instruction.params, codes, &mut cursor);
        cursor += instruction.opcode.cursor_change();
        instruction = create_instruction(cursor, codes);
    }
}
