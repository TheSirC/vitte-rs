use vitte_rs::sampler::Sampler;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn size_collection(c: &mut Criterion) {
    let mut group = c.benchmark_group("size_collection");
    let mut exponent = 4;
    for size in std::iter::from_fn(move || {
        exponent += 1;
        if exponent < 9 {
            Some(10_u64.pow(exponent))
        } else {
            None
        }
    }) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let v: Vec<u64> = (1..size).collect();
                let l = v.len();
                let _ = v.into_iter().sample(1000, l, 13).collect::<Vec<_>>();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, size_collection);
criterion_main!(benches);
