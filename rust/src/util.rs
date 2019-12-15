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
