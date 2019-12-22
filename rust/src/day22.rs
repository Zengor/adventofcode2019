#[derive(Debug)]
enum ShuffleTech {
    DealNew,
    CutN(i64),
    DealIncr(u64),
}

struct Shuffler {
    stack: Vec<u64>,
    working_table: Vec<u64>,
}

impl Shuffler {
    fn new(stack: impl Into<Vec<u64>>) -> Self {
        let stack = stack.into();
        let working_table = Vec::with_capacity(stack.len());
        Self {
            stack,
            working_table,
        }
    }

    fn shuffle_with(&mut self, technique: &ShuffleTech) {
        match *technique {
            ShuffleTech::DealNew => {
                self.stack.reverse();
            }
            ShuffleTech::CutN(n) => {
                if n >= 0 {
                    self.stack.rotate_left(n as usize);
                } else {
                    self.stack.rotate_right(n.abs() as usize);
                }
            }
            ShuffleTech::DealIncr(n) => {
                let len = self.stack.len();
                self.working_table.clear();
                self.working_table.resize(len, 0);
                let cards = self.stack.drain(..);
                for (i, c) in (0..).step_by(n as usize).take(len).zip(cards) {
                    self.working_table[i % len] = c;
                }
                std::mem::swap(&mut self.stack, &mut self.working_table);
            }
        }
    }

    fn shuffle_seq(&mut self, shuffles: &[ShuffleTech]) {
        for shuffle in shuffles.iter() {
            self.shuffle_with(shuffle);
        }
    }
}

fn parse_shuffle(input: &str) -> Vec<ShuffleTech> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut words = l.trim().split(" ");
            // the second word is enough to distinguish
            let second_word = words.nth(1).unwrap();
            match second_word {
                "into" => ShuffleTech::DealNew,
                "with" => {
                    let n = words.last().unwrap().parse().expect("Invalid deal incr");
                    ShuffleTech::DealIncr(n)
                }
                // any other is expected to be a number for the Cut
                // technique. If that's not the case it's an unknown
                // technique and panicking is appropriate.
                n => ShuffleTech::CutN(n.parse().expect("Invalid cut N")),
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let shuffles = parse_shuffle(input);
    let stack: Vec<_> = (0..10007).collect();
    let mut shuffler = Shuffler::new(stack);
    shuffler.shuffle_seq(&shuffles);
    shuffler.stack.iter().position(|c| *c == 2019).unwrap()
}

struct Deck {
    start: i128,
    increment: i128,
    size: i128,
}

impl Deck {
    fn nth(&self, n: i128) -> i128 {
        // because the operations that modify these values
        // all use `rem_euclid`, this is guaranteed to be positive
        // and the correct card, even using the default % operator
        (self.start + self.increment * n) % self.size
    }
    /// Adds start offset to given `x`, maintaining it within deck size
    fn add_offset(&mut self, x: i128) {
        self.start += x;
        self.start = self.start.rem_euclid(self.size);
    }
    /// Multiplies increment by a given x maintaining it within deck size`
    fn mul_increment(&mut self, x: i128) {
        self.increment *= x;
        self.increment = self.increment.rem_euclid(self.size);
    }

    fn shuffle_with(mut self, shuffle: &ShuffleTech) -> Deck {
        use ShuffleTech::*;
        match *shuffle {
            DealNew => {
                // This works because when the offset is inverted
                // and the modulo is taken, it'll be equal to the last
                // number of the list.
                self.mul_increment(-1);
                self.add_offset(self.increment);
            }
            DealIncr(n) => {
                let inv_mod = crate::util::modular_inverse(n as i64, self.size as i64) as i128;
                self.mul_increment(inv_mod);
            }
            CutN(n) => self.start = self.nth(n as i128),
        }
        self
    }
    fn shuffle_seq_n(mut self, shuffles: &[ShuffleTech], n: u64) -> Deck {
        // apply shuffle once
        for shuffle in shuffles.iter() {
            self = self.shuffle_with(shuffle);
        }
        // repeating this process n times means the final increment will
        // be increment ^ n mod size
        let final_inc = modular_exp(self.increment, n as i128, self.size);
        // we'll use this in the following calc
        // the final start offset is a geometric series
        // final_start = start * (1 + increment + increment^2 + ... increment^n-1)
        //
        // so final_start = start * (1-increment^n) * (1-increment)^-1
        // note that increment^n is final_inc, (1-increment)^-1 is the
        // modular inverse
        let inverse =
            crate::util::modular_inverse(1 - self.increment as i64, self.size as i64) as i128;
        let final_start = self.start * (((1 - final_inc) * inverse) % self.size);
        self.start = final_start.rem_euclid(self.size);
        self.increment = final_inc;
        self
    }
}

fn modular_exp(mut base: i128, mut exp: i128, m: i128) -> i128 {
    let mut r = 1;
    base = base % m;
    while exp > 0 {
        if exp % 2 == 1 {
            r = (r * base) % m;
        }
        exp >>= 1;
        base = base.pow(2) % m;
    }
    r
}

pub fn part2(input: &str) -> i128 {
    const SIZE: i128 = 119_315_717_514_047;
    const REPS: u64 = 101_741_582_076_661;
    let shuffles = parse_shuffle(input);
    let deck = Deck {
        start: 0,
        increment: 1,
        size: SIZE,
    };
    let deck = deck.shuffle_seq_n(&shuffles, REPS);
    deck.nth(2020)
}
