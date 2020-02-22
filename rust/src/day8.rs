use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part1(input: &str) -> usize {
    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize);
    let layer_counts = digits
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|layer| {
            // there are only 3 possible digits
            let mut counts = vec![0; 3];
            for d in layer {
                counts[d] += 1;
            }
            counts
        })
        .min_by_key(|c| c[0])
        .unwrap();
    layer_counts[1] * layer_counts[2]
}

#[derive(Debug, Clone, Copy)]
enum PixelColor {
    Black,
    White,
    Transparent,
}

impl From<char> for PixelColor {
    fn from(c: char) -> Self {
        match c {
            '0' => PixelColor::Black,
            '1' => PixelColor::White,
            '2' => PixelColor::Transparent,
            _ => panic!("Invalid pixel digit!"),
        }
    }
}

fn get_color(layers: &[PixelColor], idx: usize, num_layers: usize) -> PixelColor {
    for l in 0..num_layers {
        match layers[idx + (l * WIDTH * HEIGHT)] {
            PixelColor::Transparent => continue,
            c => return c,
        }
    }
    PixelColor::Transparent
}

pub fn part2(input: &str) {
    let image_len = input.len();
    let num_layers = (image_len / WIDTH) * HEIGHT;
    //let mut layers = vec![Vec::with_capacity(image_len); num_layers];

    let layers: Vec<PixelColor> = input.trim().chars().map(PixelColor::from).collect();

    for i in 0..(WIDTH * HEIGHT) {
        match get_color(&layers, i, num_layers) {
            PixelColor::Black => print!("▓"),
            PixelColor::White => print!("░"),
            PixelColor::Transparent => print!("  "),
        }
        if (i + 1) % WIDTH == 0 {
            println!();
        }
    }
}
