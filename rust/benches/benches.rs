use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! bench_func {
    ($group:ident, $name:expr, $func:expr, $input:expr) => {
        $group.bench_function($name, |b| b.iter(|| $func(black_box($input))));
    };
}

fn day1(c: &mut Criterion) {
    use adventofcode2019::day1::*;
    
    let input = include_str!("../../input/01-1.txt");
    let mut group = c.benchmark_group("Day 1");
        
    bench_func!(group, "Part1", part1, &input);
    bench_func!(group, "Part2", part2, &input);
    let input: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    group.bench_with_input(BenchmarkId::new("Part1", "noparse"), &input, |b, input| {
        b.iter(|| {
            input
                .iter()
                .map(|i| adventofcode2019::day1::fuel_needed(black_box(*i)))
                .sum::<i32>()
        })
    });
    group.bench_with_input(BenchmarkId::new("Part2", "noparse"), &input, |b, input| {
        b.iter(|| {
            input
                .iter()
                .map(|i| adventofcode2019::day1::fuel_needed(black_box(*i)))
                .sum::<i32>()
        })
    });
}
//     ($ident:item, $group_name:expr, $input:expr, $($fun:item),*) => {
//         let input = include_str!(concat!("../../input/", $input, ".txt"));
//         let mut group = c.benchmark_group("Day5");
//         $ (
//             group.bench_function(concat!(
//         );
//     }
// }


fn day5(c: &mut Criterion) {
    use adventofcode2019::day5::*;
    let input = include_str!("../../input/05-1.txt");
    let mut group = c.benchmark_group("Day 5");
    bench_func!(group, "Part1", part1, &input);
    bench_func!(group, "Part2", part2, &input);
}

fn day3(c: &mut Criterion) {
    use adventofcode2019::day3::*;
    let input = include_str!("../../input/03-1.txt");
    let mut group = c.benchmark_group("Day 3");
    let a = input.split(",");
    group.bench_with_input(BenchmarkId::from_parameter("vec"), &a, |b, a| {
        b.iter(move || {
            let _v: Wire = points_from_segments(black_box(a.clone()));
        })
    });
    bench_func!(group, "Part1", part1, &input);
    bench_func!(group, "Part2", part2, &input);
}

// fn day5(c: &mut Criterion) {
//     let input = include_str!("../../input/05-1.txt");
//     let mut group = c.benchmark_group("Day 5");

//     group.bench_function("Part1", |b| {
//         b.iter(|| {
//             adventofcode2019::day5::part1(black_box(&input));
//         })
//     });
//     group.bench_function("Part2", |b| {
//         b.iter(|| {
//             adventofcode2019::day5::part2(black_box(&input));
//         })
//     });
// }

fn day6(c: &mut Criterion) {
    use adventofcode2019::day6::*;
    
    let input = include_str!("../../input/06-1.txt");
    let mut group = c.benchmark_group("Day 6");
    
    bench_func!(group, "Part1", part1, &input);
    bench_func!(group, "Part2", part2, &input);    
    // bench_func!(group, "Part1/graphmap", part1_graphmap, &input);
    // bench_func!(group, "Part2/graphmap", part2_graphmap, &input);
}

fn day7(c: &mut Criterion) {
    use adventofcode2019::day7::*;
    let input = include_str!("../../input/07-1.txt");
    let mut group = c.benchmark_group("Day 7");
    
    bench_func!(group, "Part1", part1, &input);
    bench_func!(group, "Part2", part2, &input);
}

criterion_group!(benches, day1, day3, day5, day6, day7);
criterion_main!(benches);
