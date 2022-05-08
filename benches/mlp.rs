use std::alloc::Layout;

use benchmarks::mlp::ptr_chase;
use benchmarks::mlp::Ptr;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use rand::prelude::*;

fn gen_random_chase(count: usize, stride: usize, seed: u64) -> Ptr {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    if stride < std::mem::size_of::<Ptr>() {
        panic!("stride must be at least {}", std::mem::size_of::<Ptr>());
    }
    if stride % std::mem::size_of::<Ptr>() != 0 {
        panic!(
            "stride must be a multiple of {}",
            std::mem::size_of::<Ptr>()
        );
    }
    let bytes = stride * count;
    let ptrs_in_a_stride = stride / std::mem::size_of::<Ptr>();
    const MIXER_COUNT: usize = 16384;
    let mut mixers = vec![0; MIXER_COUNT];
    for i in 0..MIXER_COUNT {
        let mut indices: Vec<_> = (0..ptrs_in_a_stride).collect();
        indices.shuffle(&mut rng);
        mixers[i] = indices[0];
    }
    let layout = Layout::from_size_align(bytes, std::mem::size_of::<Ptr>()).unwrap();
    let arena = unsafe { std::alloc::alloc_zeroed(layout) as *mut Ptr };
    let mut permutation: Vec<_> = (0..count).collect();
    permutation.shuffle(&mut rng);
    let mut inverse_permutation = vec![0; count];
    for i in 0..count {
        inverse_permutation[permutation[i]] = i;
    }
    for i in 0..count {
        let next = inverse_permutation[i] + 1;
        let next = if next == count { 0 } else { next };
        unsafe {
            let ptr = arena
                .add(i * ptrs_in_a_stride + mixers[i % MIXER_COUNT]);
            let next_ptr = arena.add(permutation[next] * ptrs_in_a_stride + mixers[permutation[next] % MIXER_COUNT]) as Ptr;
            ptr.write(next_ptr);
        };
    }
    unsafe { arena.add(mixers[0]) as Ptr }
}

fn mlp(c: &mut Criterion) {
    let mut group = c.benchmark_group("mlp");
    let count = 1024 * 1024 * 32; // 2048MB
    let cores = core_affinity::get_core_ids().unwrap();
    let ptrs = gen_random_chase(count, 64, 192608179845);
    core_affinity::set_for_current(cores[0]);
    {
        let mut p = ptrs;
        group
            .throughput(criterion::Throughput::Elements(200))
            .bench_function("random_ptr_chase", |b| {        
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
