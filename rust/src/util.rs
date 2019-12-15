#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {    
    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    pub fn turn_left(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    pub fn tuple(&self) -> (i32, i32) {
        use Direction::*;
        match *self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}


pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let old_b = b;
        b = a % b;
        a = old_b;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
