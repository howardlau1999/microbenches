use benchmarks::mlp::ptr_chase;
use benchmarks::mlp::PaddedPtr;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use rand::prelude::*;

fn mlp(c: &mut Criterion) {
    let mut group = c.benchmark_group("mlp");
    let count = 1024 * 1024 * 32; // 2048MB
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut ptrs: Vec<_> = (0..count)
        .map(|_| {
            let ptr = PaddedPtr {
                next: 0 as *const PaddedPtr,
                padding: [0; 7],
            };
            ptr
        })
        .collect();
    let mut shuffled = (0..count).collect::<Vec<_>>();
    shuffled.shuffle(&mut rng);
    ptrs[shuffled[(count - 1) as usize] as usize].next = &ptrs[shuffled[0 as usize] as usize];
    for i in 1..count {
        ptrs[shuffled[(i - 1) as usize] as usize].next = &ptrs[shuffled[i as usize] as usize];
    }
    {
        group
            .throughput(criterion::Throughput::Elements(200))
            .bench_function("random_ptr_chase", |b| {
                let mut p = ptrs.as_ptr();
                b.iter(|| {
                    unsafe { ptr_chase(&mut p) };
                })
            });
    }
    for i in 0..count {
        ptrs[i as usize].next = &ptrs[(i + 1) % count];
    }
    {
        group
            .throughput(criterion::Throughput::Elements(200))
            .bench_function("seq_ptr_chase", |b| {
                let mut p = ptrs.as_ptr();
                b.iter(|| {
                    unsafe { ptr_chase(&mut p) };
                })
            });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = mlp
);
criterion_main!(benches);
