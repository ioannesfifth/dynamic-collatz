use collatz::run_collatz;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion, Bencher
};

fn collatz_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("collatz");
    group.sample_size(10);
    group.bench_function(
        "collatz",
        |b: &mut Bencher| b.iter(|| run_collatz())
    );
}

criterion_group!(benches, collatz_benchmark);
criterion_main!(benches);