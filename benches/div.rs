use benchmarks::div_f64;
use benchmarks::div_u64;
use criterion::criterion_main;
use criterion::criterion_group;
use criterion::Criterion;
use criterion_cycles_per_byte::CyclesPerByte;

fn div(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("div");
    group.bench_function("u64", |b| b.iter(div_u64));
    group.bench_function("f64", |b| b.iter(div_f64));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = div
);
criterion_main!(benches);