use std::{collections::VecDeque, io::Write};

pub trait IntcodeInput {
    fn read(&mut self) -> Option<i64>;
}

pub trait IntcodeOutput {
    fn write(&mut self, out: i64);
}

impl IntcodeInput for std::io::Empty {
    fn read(&mut self) -> Option<i64> {
        None
    }
}

impl IntcodeInput for std::io::Lines<std::io::StdinLock<'_>> {
    fn read(&mut self) -> Option<i64> {
        self.next()?.ok()?.parse().ok()
    }
}

impl IntcodeInput for &str {
    fn read(&mut self) -> Option<i64> {
        self.lines().next()?.parse().ok()
    }
}

impl IntcodeInput for Vec<i64> {
    fn read(&mut self) -> Option<i64> {
        if self.len() == 0 {
            return None;
        }
        Some(self.remove(0))
    }
}

impl IntcodeInput for Option<i64> {
    fn read(&mut self) -> Option<i64> {
        self.take()
    }
}

impl IntcodeInput for std::iter::Repeat<i64> {
    fn read(&mut self) -> Option<i64> {
        self.next()
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

pub struct AsciiTranslator {
    string: std::collections::VecDeque<char>,
}

impl AsciiTranslator {
    pub fn new() -> Self {
        Self {
            string: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.string.clear();
    }

    pub fn push_string(&mut self, s: String) {
        self.string.extend(s.chars());
        self.string.push_back('\n');
    }

    pub fn drain_string(&mut self) -> String {
        self.string.drain(..).collect()
    }
}

impl IntcodeInput for AsciiTranslator {
    fn read(&mut self) -> Option<i64> {
        self.string.pop_front().map(|c| c as i64)
    }
}

impl IntcodeOutput for AsciiTranslator {
    fn write(&mut self, out: i64) {
        self.string.push_back(out as u8 as char)
    }
}
