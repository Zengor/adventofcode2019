//! # Day 3: Crossed Wires
//!
//! ## Problem Description
//!
//! Given sequences of steps that form two wires in a grid, find the intersection
//! of the wires that is closest to the center. In part 1, this is
//! done by manhattan distance, in part 2 by signal distance (how long
//! it takes the wire to get to that position).
//!
//! ## Implementention details
//!
//! Wires are created as a Vec containing all the points they cover in
//! sequence. The wires are converted into HashSets to find all
//! intersections, then the distance function is applied to those
//! points to find the closest. For part 2, the distance is simply the
//! index of that point in the Wire's Vec.
//!
//! I had done an implementation where in part 1 the wires were
//! collected directly into a HashSet without going through the Vec
//! first, but for some reason that turned out slower than creating
//! the Vec first, then creating the HashSet from it.

use itertools::Itertools;
use std::collections::HashSet;

use crate::util::Point;

fn dir_tuple(s: char) -> (i32, i32) {
    match s {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn manhattan_dist(a: &Point, b: &Point) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

pub struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn dist_from_start(&self, point: &Point) -> usize {
        self.points.iter().position(|x| x == point).unwrap() + 1 as usize
    }
    fn hashset(&self) -> HashSet<Point> {
        self.points.iter().cloned().collect()
    }
}

pub fn points_from_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> Wire {
    let segments = segments.into_iter();
    let mut dir_mods = segments.flat_map(|segment| {
        let (x, y) = dir_tuple(segment.chars().nth(0).unwrap());
        let steps = segment[1..].parse().unwrap();
        std::iter::repeat((x, y)).take(steps)
    });
    let points = itertools::unfold(Point::new(0, 0), move |curr_pos| {
        let (x, y) = dir_mods.next()?;
        curr_pos.x = curr_pos.x + x;
        curr_pos.y = curr_pos.y + y;
        Some(curr_pos.clone())
    })
    .collect();
    Wire { points }
}

fn parse_wires(input: &str) -> (Wire, Wire) {
    input
        .lines()
        .map(|w| points_from_segments(w.split(',')))
        .collect_tuple()
        .unwrap()
}

fn find_closest_intersection(a: &Wire, b: &Wire, dist: impl Fn(&Point) -> usize) -> usize {
    let (a, b) = (a.hashset(), b.hashset());
    a.intersection(&b).map(|p| dist(p)).min().unwrap()
}

pub fn part1(input: &str) -> usize {
    let (wire_a, wire_b) = parse_wires(input);
    let origin = Point::new(0, 0);
    let dist = |p: &Point| manhattan_dist(&origin, p);
    find_closest_intersection(&wire_a, &wire_b, dist)
}

pub fn part2(input: &str) -> usize {
    let (wire_a, wire_b) = parse_wires(input);
    let dist = |p: &Point| wire_a.dist_from_start(p) + wire_b.dist_from_start(p);
    find_closest_intersection(&wire_a, &wire_b, dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const EX2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const EX3: &str =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    const INPUT: &str = include_str!("../../input/03-1.txt");
    #[test]
    fn test_distances() {
        let origin = Point::new(0, 0);
        let points: Vec<_> = [(0, 5), (5, 0), (-5, 0), (0, -5), (5, 5)]
            .into_iter()
            .map(|p| Point::new(p.0, p.1))
            .collect();
        assert_eq!(5, manhattan_dist(&origin, &points[0]));
        assert_eq!(5, manhattan_dist(&origin, &points[1]));
        assert_eq!(5, manhattan_dist(&origin, &points[2]));
        assert_eq!(5, manhattan_dist(&origin, &points[3]));
        assert_eq!(10, manhattan_dist(&origin, &points[4]));
        let points: Vec<_> = [
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (-1, 4),
            (-2, 4),
            (-3, 4),
            (-4, 4),
            (-4, 3),
            (-3, 3),
            (-3, 2),
            (-3, 1),
            (-4, 1),
            (-5, 1),
            (-5, 0),
            (-5, -1),
            (-6, -1),
            (-6, -2),
            (-6, -3),
        ]
        .into_iter()
        .map(|p| Point::new(p.0, p.1))
        .collect();
        let wire = Wire {
            points: points.clone(),
        };
        assert_eq!(1, wire.dist_from_start(&points[0]));
        assert_eq!(10, wire.dist_from_start(&points[9]));
        assert_eq!(11, wire.dist_from_start(&points[10]));
        assert_eq!(16, wire.dist_from_start(&points[15]));
        assert_eq!(19, wire.dist_from_start(&points[18]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(EX1));
        assert_eq!(159, part1(EX2));
        assert_eq!(135, part1(EX3));
        assert_eq!(5357, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(EX1));
        assert_eq!(610, part2(EX2));
        assert_eq!(410, part2(EX3));
        assert_eq!(101956, part2(INPUT));
    }
}
