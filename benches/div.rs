use criterion::criterion_main;
use criterion::criterion_group;
use criterion::Criterion;
use criterion::black_box;
use criterion_cycles_per_byte::CyclesPerByte;

fn div(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("div");
    group.bench_function("u64", |b| b.iter(|| {
        let result = black_box(64 as u64) / black_box(3 as u64);
        black_box(result);
    }));
    group.bench_function("f32", |b| b.iter(|| {
        let result = black_box(64 as f32) / black_box(3 as f32);
        black_box(result);
    }));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = div
);
criterion_main!(benches);