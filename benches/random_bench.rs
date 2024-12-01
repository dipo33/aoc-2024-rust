use criterion::{criterion_group, criterion_main, Criterion};

fn do_something(str: &str) -> &str {
    &str[1..]
}

fn copied_in_iter<'a, I>(iter: I) -> Vec<&'a str>
where
    I: Iterator<Item = &'a &'a str>,
{
    iter.copied().map(do_something).collect()
}

fn explicit_closure<'a, I>(iter: I) -> Vec<&'a str>
where
    I: Iterator<Item = &'a &'a str>,
{
    iter.map(|str| do_something(str)).collect()
}

fn benchmark(c: &mut Criterion) {
    let data = (1..10000).map(|x| x.to_string()).collect::<Vec<_>>();
    let data_str = data.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    c.bench_function("With copied()", |b| {
        b.iter(|| copied_in_iter(data_str.iter()))
    });

    c.bench_function("With explicit closure", |b| {
        b.iter(|| explicit_closure(data_str.iter()))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
