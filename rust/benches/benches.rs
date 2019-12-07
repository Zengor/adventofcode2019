use criterion::{black_box, criterion_group, criterion_main, Criterion};

const FILE: &'static str = "../input/06-1.txt";

fn day5(c: &mut Criterion) {
    let input = include_str!("../../input/05-1.txt");
    c.bench_function("Day 5 Part 1", |b| {
        b.iter(|| {
            adventofcode2019::day5::part1(black_box(&input));
        })
    });
    c.bench_function("Dat 5 Part 2", |b| {
        b.iter(|| {
            adventofcode2019::day5::part2(black_box(&input));
        })
    });
}

fn day6(c: &mut Criterion) {
    let input = include_str!("../../input/06-1.txt");
    {
        let mut group = c.benchmark_group("Day 6 Part 1");

        group.bench_function("graph", |b| {
            b.iter(|| {
                adventofcode2019::day6::part1(black_box(&input));
            })
        });
        group.bench_function("graphmap", |b| {
            b.iter(|| {
                adventofcode2019::day6::part1_graphmap(black_box(&input));
            })
        });
    }
    {
        let mut group = c.benchmark_group("Day 6 Part 2");
        group.bench_function("graph", |b| {
            b.iter(|| {
                adventofcode2019::day6::part2(black_box(&input));
            })
        });
        group.bench_function("graphmap", |b| {
            b.iter(|| {
                adventofcode2019::day6::part2_graphmap(black_box(&input));
            })
        });
    }
}

fn day7(c: &mut Criterion) {
    let input = include_str!("../../input/07-1.txt");
    c.bench_function("Day 7 Part 1", |b| {
        b.iter(|| {
            adventofcode2019::day7::part1(black_box(&input))
        })
    });
    c.bench_function("Dat 7 Part 2", |b| {
        b.iter(|| {
            adventofcode2019::day7::part2(black_box(&input))
        })
    });
}

criterion_group!(benches, day5, day6, day7);
criterion_main!(benches);
