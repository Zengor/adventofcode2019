//use util::Point;
use crate::util::{gcd, Point};
use std::collections::HashMap;

fn simplify(x: i32, y: i32) -> (i32, i32) {
    let divisor = gcd(x as i64, y as i64).abs();
    (x / divisor as i32, y / divisor as i32)
}

// Using the same function for part 1 and part 2, so this needs to
// return all other asteroids that are visible at a given angle. This
// is not necessary for part 1, as all it needs is how many separate
// angles have any visible asteroids at all so for a part1-only
// solution it would be more space efficient to just use a hashset of
// angles.
fn get_visible<'a>(asteroid: &Point, field: &'a [Point]) -> HashMap<(i32, i32), Vec<Point>> {
    let mut vis: HashMap<(_, _), Vec<Point>> = HashMap::new();
    for other in field.iter().filter(|&o| o != asteroid) {
        let angle = simplify(other.x - asteroid.x, other.y - asteroid.y);
        vis.entry(angle).or_default().push(other.clone());
    }
    vis
}

fn parse_asteroids(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    asteroids
        .iter()
        .map(|a| get_visible(a, &asteroids).len())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i32 {
    let asteroids = parse_asteroids(input);
    // first solving part 1 to not have to hardcode the coordinates
    // for the station used in part 2
    let (station, mut visibility) = asteroids
        .iter()
        .map(|a| (a, get_visible(a, &asteroids)))
        .max_by_key(|(_, v)| v.len())
        .unwrap();
    let mut angles: Vec<((i32, i32), f32)> = Vec::new();
    for angle in visibility.keys() {
        let atan2 = (angle.1 as f32).atan2(angle.0 as f32);
        angles.push((*angle, atan2));
    }
    // get a cycling iterator of all angles in order, starting at angle (0,-1)
    angles.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let mut angles = angles
        .into_iter()
        .cycle()
        .skip_while(|(angle, _)| !(angle.0 == 0 && angle.1 == -1));
    // Here I'm sorting the Vecs so that the stations are actually in
    // order from furthest to closest. This is likely not necessary
    // for a lot of problem instances (including mine) as there are
    // more angles to go through than the number of asteroids we have
    // to destroy (thus it never completes a full cycle), but I decided
    // to do a more general case for this implementation anyway.
    //
    // They're in decreasing order of distance so that we can
    // just remove asteroids from the end of the Vec instead
    // of doing costly front removals or switching to VecDeques
    // switching to vecdeques for quick head removal
    for visible in visibility.values_mut() {
        visible.sort_by_key(|ast| -((station.x - ast.x).abs() + (station.y - ast.y).abs()));
    }
    let mut destroyed_count = 0;
    while destroyed_count < 199 {
        // Now the process is simple: get the next angle in order, and if
        // there is an asteroid still visible at that angle, remove it.
        let (curr_angle, _) = angles.next().unwrap();
        if let Some(visible) = visibility.get_mut(&curr_angle) {
            if !visible.is_empty() {
                visible.pop();
                destroyed_count += 1;
            }
        }
    }
    let (final_angle, _) = angles.next().unwrap();
    let result = &visibility[&final_angle][0];
    return 100 * result.x + result.y;
}
