use std::io::{Write};

pub trait IntcodeInput {
    fn read_line(&mut self) -> Option<String>;
}

pub trait IntcodeOutput {
    fn write(&mut self, out: i64);
}

impl IntcodeInput for std::io::Empty {
    fn read_line(&mut self) -> Option<String> {
        None
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

impl IntcodeInput for Vec<i64> {
    fn read_line(&mut self) -> Option<String> {
        if self.len() == 0 {
            return None;
        }
        Some(self.remove(0).to_string())
    }
}

impl IntcodeInput for Option<i64> {
    fn read_line(&mut self) -> Option<String> {
        self.take().map(|i| i.to_string())
    }
}

impl IntcodeOutput for std::io::Sink {
    fn write(&mut self, _: i64) {}
}
impl IntcodeOutput for std::io::StdoutLock<'_> {
    fn write(&mut self, out: i64) {
        match writeln!(self, "OUTPUT: {}", out) {
            // silently fail
            _ => (),
        };
    }
}
impl IntcodeOutput for Vec<i64> {
    fn write(&mut self, out: i64) {
        self.push(out)
    }
}

impl IntcodeOutput for Option<i64> {
    fn write(&mut self, out: i64) {
        *self = Some(out);
    }
}

