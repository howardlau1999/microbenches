use criterion::criterion_main;
use criterion::criterion_group;
use criterion::Criterion;
use criterion::black_box;
use criterion_cycles_per_byte::CyclesPerByte;
use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};
use simd::i32x4;

fn simd_sum(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("simd_sum");
    let between = Uniform::from(10..100);
    let mut rng = rand::thread_rng();
    let simd_arr: Vec<_> = (0..100000).map(|_| i32x4::new(
        between.sample(&mut rng),
        between.sample(&mut rng),
        between.sample(&mut rng),
        between.sample(&mut rng))
    ).collect();
    group.bench_function("unpack", |b| b.iter(|| {
        let mut sum = i32x4::new(0,0,0,0);
        for v in simd_arr {
            sum += v;
        }
        let sum = sum[0] + sum[1] + sum[2] + sum[3];
        black_box(sum);
    }));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = simd_sum
);
criterion_main!(benches);