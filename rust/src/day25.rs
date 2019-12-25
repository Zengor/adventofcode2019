use crate::intcode::{AsciiTranslator, IntcodeMachine};
use std::io::BufRead;

// struct Droid {
//     program: IntcodeMachine,
// }

// impl Droid {
//     fn new(source_code: &str) -> Self {
//         Self {
//             program: IntcodeMachine::from_str(source_code),
//         }
//     }
// }

pub fn part1(input: &str) {
    //let mut droid = Droid::new(input);
    let mut droid = IntcodeMachine::from_str(input);
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut input = AsciiTranslator::new();
    let mut output = AsciiTranslator::new();
    droid.run_while_input(&mut input, &mut output);
    println!("{}", &output.drain_string());
    for line in stdin.lines().map(|l| l.unwrap()) {
        if line.contains("!quit!") {
            break;
        }
        input.push_string(line);
        output.clear();
        droid.run_while_input(&mut input, &mut output);
        println!("{}", &output.drain_string());
    }
}
