use crate::intcode::IntcodeMachine;
use itertools::iproduct;

struct BeamDetector {
    program: IntcodeMachine,
}

impl BeamDetector {
    fn within_beam(&self, x: i64, y: i64) -> bool {
        let mut program = self.program.clone();
        let mut input = vec![x, y];
        let mut out = None;
        program.run_while_input(&mut input, &mut out);
        match out {
            Some(1) => true,
            _ => false,
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let detector = BeamDetector {
        program: IntcodeMachine::from_str(input),
    };
    let mut count = 0;
    for (y, x) in iproduct!(0..50, 0..50) {
        if detector.within_beam(x, y) {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> i64 {
    let detector = BeamDetector {
        program: IntcodeMachine::from_str(input),
    };
    // given this example (looking for a 4x4)
    // . . . . B
    // . . . . .
    //   . . . . .
    //   A . . . . .
    // we're walking until we find an A such that B is also within the beam.

    // starting at y = 99 because the square has to be 100 tall
    let (mut x, mut y) = (0, 99);
    loop {
        // continue walking right until a filled in this line
        if !detector.within_beam(x, y) {
            x += 1;
            continue;
        }
        // if B is not within the beam, we walk down to the next line
        if !detector.within_beam(x + 99, y - 99) {
            y += 1;
            continue;
        }
        // the answer is based on the x,y of the top left, so we have
        // to reduce y to get the right value
        return 10000 * x + y - 99;
    }
}
