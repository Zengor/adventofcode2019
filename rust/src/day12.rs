use itertools::Itertools;

use std::collections::HashSet;

fn axis_change(a: i32, b: i32) -> (i32, i32) {
    use std::cmp::Ordering::*;
    match a.cmp(&b) {
        Less => (1, -1),
        Greater => (-1, 1),
        Equal => (0, 0),
    }
}

fn get_position_axes(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut x_positions = Vec::with_capacity(4);
    let mut y_positions = Vec::with_capacity(4);
    let mut z_positions = Vec::with_capacity(4);

    for l in input.lines() {
        let mut l = l
            .trim_matches(|c| c == '<' || c == '>')
            .split(",")
            .map(|s| s.trim()[2..].parse().unwrap());
        x_positions.push(l.next().unwrap());
        y_positions.push(l.next().unwrap());
        z_positions.push(l.next().unwrap());
    }
    (x_positions, y_positions, z_positions)
}

fn sim_step(positions: &mut [i32], velocities: &mut [i32]) {
    for (a, b) in (0..positions.len()).tuple_combinations() {
        let (change_a, change_b) = axis_change(positions[a], positions[b]);
        velocities[a] += change_a;
        velocities[b] += change_b;
    }

    for p in 0..positions.len() {
        positions[p] += velocities[p]
    }
}

pub fn part1(input: &str) -> i32 {
    let (mut x_positions, mut y_positions, mut z_positions) = get_position_axes(input);
    let num_particles = x_positions.len();
    let mut x_velocities = vec![0; num_particles];
    let mut y_velocities = vec![0; num_particles];
    let mut z_velocities = vec![0; num_particles];

    let timesteps = 1000;
    for _ in 0..timesteps {
        sim_step(&mut x_positions, &mut x_velocities);
        sim_step(&mut y_positions, &mut y_velocities);
        sim_step(&mut z_positions, &mut z_velocities);
    }
    (0..num_particles)
        .map(|i| {
            let pot = x_positions[i].abs() + y_positions[i].abs() + z_positions[i];
            let kin = x_velocities[i].abs() + y_velocities[i].abs() + z_velocities[i].abs();
            pot * kin
        })
        .sum()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let old_b = b;
        b = a % b;
        a = old_b;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn part2(input: &str) -> i64 {
    let (x_positions, y_positions, z_positions) = get_position_axes(input);

    let x_repeat = find_axis_repeat(x_positions);
    let y_repeat = find_axis_repeat(y_positions);
    let z_repeat = find_axis_repeat(z_positions);

    lcm(lcm(x_repeat, y_repeat), z_repeat)
}

fn find_axis_repeat(mut positions: Vec<i32>) -> i64 {
    let start = positions.clone();
    let mut velocities = vec![0; positions.len()];
    let mut step = 0;

    loop {
        step += 1;
        sim_step(&mut positions, &mut velocities);
        if velocities.iter().all(|v| *v == 0) && (start == positions) {
            println!("--{:?} {:?}", &positions, &velocities);
            return dbg!(step);
        }
    }
}
