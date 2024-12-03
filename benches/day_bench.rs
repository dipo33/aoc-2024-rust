use aoc_2024_rust::days::{
    day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04, day_05::Day05, day_06::Day06,
    day_07::Day07, day_08::Day08, day_09::Day09, day_10::Day10, day_11::Day11, day_12::Day12,
    day_13::Day13, day_14::Day14, day_15::Day15, day_16::Day16, day_17::Day17, day_18::Day18,
    day_19::Day19, day_20::Day20, day_21::Day21, day_22::Day22, day_23::Day23, day_24::Day24,
    day_25::Day25, day_26::Day26, day_27::Day27, day_28::Day28, day_29::Day29, day_30::Day30,
};
use aoc_common::{day::Day, input_fetcher};
use criterion::{criterion_group, criterion_main, Criterion};
use dotenvy::dotenv;

fn benchmark_day<T: Day + Default>(c: &mut Criterion) {
    dotenv().ok();
    let day = T::default();
    let day_number = day.get_day();
    let input = input_fetcher::get_or_fetch_input(day_number)
        .expect(format!("Failed to fetch input for day {:02}", day_number).as_str());

    c.bench_function(&format!("Day {:02} - Part 1", day.get_day()), |b| {
        b.iter(|| day.solve_part_1(&input))
    });

    c.bench_function(&format!("Day {:02} - Part 2", day.get_day()), |b| {
        b.iter(|| day.solve_part_2(&input))
    });
}

fn benchmark_day1(c: &mut Criterion) {
    benchmark_day::<Day01>(c);
    benchmark_day::<Day02>(c);
    benchmark_day::<Day03>(c);
    benchmark_day::<Day04>(c);
    benchmark_day::<Day05>(c);
    benchmark_day::<Day06>(c);
    benchmark_day::<Day07>(c);
    benchmark_day::<Day08>(c);
    benchmark_day::<Day09>(c);
    benchmark_day::<Day10>(c);
    benchmark_day::<Day11>(c);
    benchmark_day::<Day12>(c);
    benchmark_day::<Day13>(c);
    benchmark_day::<Day14>(c);
    benchmark_day::<Day15>(c);
    benchmark_day::<Day16>(c);
    benchmark_day::<Day17>(c);
    benchmark_day::<Day18>(c);
    benchmark_day::<Day19>(c);
    benchmark_day::<Day20>(c);
    benchmark_day::<Day21>(c);
    benchmark_day::<Day22>(c);
    benchmark_day::<Day23>(c);
    benchmark_day::<Day24>(c);
    benchmark_day::<Day25>(c);
    benchmark_day::<Day26>(c);
    benchmark_day::<Day27>(c);
    benchmark_day::<Day28>(c);
    benchmark_day::<Day29>(c);
    benchmark_day::<Day30>(c);
}

criterion_group!(benches, benchmark_day1);
criterion_main!(benches);
