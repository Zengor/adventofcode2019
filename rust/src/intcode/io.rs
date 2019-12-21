use std::io::Write;

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

pub struct IOWrapper<'a> {
    inner: &'a [i64],
    cursor: usize,
}

impl<'a> IntcodeInput for IOWrapper<'a> {
    fn read(&mut self) -> Option<i64> {
        unimplemented!()
    }
}
