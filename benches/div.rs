use benchmarks::div::div_f64;
use benchmarks::div::div_u64;
use criterion::criterion_main;
use criterion::criterion_group;
use criterion::Criterion;

fn div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");
    let cores = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(cores[0]);
    group.bench_function("u64", |b| b.iter(div_u64));
    group.bench_function("f64", |b| b.iter(div_f64));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = div
);
criterion_main!(benches);