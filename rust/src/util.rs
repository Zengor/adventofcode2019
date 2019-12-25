use std::{
    iter::Extend,
    ops::{Add, Index, IndexMut, Sub},
};

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

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Clone, Copy)]
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

/// Computers the modular inverse of a mod b by Extended Euclidean algorithm.
/// Panics if it doesn't exist.
///
/// Based on https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
pub fn modular_inverse(a: i64, b: i64) -> i64 {
    let (mut t_k, mut t_next) = (0,1);
    let (mut r_k, mut r_next) = (b,a);
    while r_next != 0 {
        let quot = r_k / r_next;
        let temp_r = r_k - quot * r_next;
        let temp_t = t_k - quot * t_next;
        r_k = r_next;
        r_next = temp_r;
        t_k = t_next;
        t_next = temp_t;
        
    }
    if r_k > 1 {
        panic!("No modular inverse could be found");
    }
    if t_k < 0 {
        t_k += b;
    }
    t_k
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
/// A very simple implementation of a matrix abstraction. It's
/// simply a Vec an associated 'width' that is used for indexing.
#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn wrap(inner: Vec<T>, width: usize) -> Self {
        Matrix { data: inner, width }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
    pub fn row(&self, i: usize) -> &[T] {
        return &self.data[(i * self.width)..(i * self.width + self.width)];
    }

    /// Returns a Matrix with memory allocated for size*size elements.
    pub fn with_capacity(size: usize) -> Matrix<T> {
        Matrix {
            data: Vec::with_capacity(size * size),
            width: size,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Provides a view into the inner vector, e.g. for cases where all
    /// elements need to be iterated in order
    pub fn inner(&self) -> &[T] {
        &self.data
    }
}

impl<T: Clone> Matrix<T> {
    /// Creates a size*size Matrix with the given initial element
    /// occupying all positions.
    pub fn with_element(size: usize, element: T) -> Matrix<T> {
        Matrix {
            data: vec![element; size * size],
            width: size,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let i = y * self.width + x;
        return &self.data[i];
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let i = y * self.width + x;
        return &mut self.data[i];
    }
}

impl<T> Extend<T> for Matrix<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter.into_iter() {
            self.push(elem)
        }
    }
}
