use aoc_2024_rust::days::day_01::Day01;
use aoc_common::{day::Day, input_fetcher};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_day<T: Day + Default>(c: &mut Criterion) {
    let day = T::default();
    let day_number = day.get_day();
    let input = input_fetcher::get_or_fetch_input(day_number)
        .expect(format!("Failed to fetch input for day {:02}", day_number).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    c.bench_function(&format!("Day {} - Part 1", day.get_day()), |b| {
        b.iter(|| day.solve_part_1(&input_lines))
    });

    c.bench_function(&format!("Day {} - Part 2", day.get_day()), |b| {
        b.iter(|| day.solve_part_2(&input_lines))
    });
}

fn benchmark_day1(c: &mut Criterion) {
    benchmark_day::<Day01>(c);
}

criterion_group!(benches, benchmark_day1);
criterion_main!(benches);
