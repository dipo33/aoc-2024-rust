use aoc_2024_rust::days::day_02::{parse_input, Day02};
use aoc_common::{day::Day, input_fetcher};
use criterion::{criterion_group, criterion_main, Criterion};
use dotenvy::dotenv;

fn benchmark_parse(c: &mut Criterion) {
    dotenv().ok();
    let day = Day02::default();
    let input = input_fetcher::get_or_fetch_input(day.get_day())
        .expect(format!("Failed to fetch input for day {:02}", day.get_day()).as_str());

    c.bench_function(&format!("Day {:02} - Parse", day.get_day()), |b| {
        b.iter(|| parse_input(&input))
    });

    c.bench_function(
        &format!("Day {:02} - Parse & Iterate", day.get_day()),
        |b| b.iter(|| parse_input(&input).map(|it| it.sum::<u32>()).sum::<u32>()),
    );
}

fn benchmarks(c: &mut Criterion) {
    benchmark_parse(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
