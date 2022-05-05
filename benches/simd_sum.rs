#![feature(portable_simd)]

use benchmarks::simd_sum::{
    auto_simd, avx_simd_256, no_simd, no_simd_unroll4, std_simd_128, std_simd_256,
    std_simd_256_unroll4, std_simd_256_unroll4x2, std_simd_256_unroll4x4,
};
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::alloc::Layout;

fn simd_sum(c: &mut Criterion) {
    let between = Uniform::from(10..100);
    let mut rng = thread_rng();
    let count = 1024 * 1024; // 32M
    let layout = Layout::from_size_align(count * std::mem::size_of::<i32>(), 64).unwrap();
    let arr = unsafe { std::alloc::alloc(layout) as *mut i32 };
    for i in 0..count {
        unsafe {
            *arr.add(i) = between.sample(&mut rng);
        }
    }
    let cores = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(cores[0]);

    let mut group = c.benchmark_group("simd_sum");
    let group = group.throughput(criterion::Throughput::Elements(count as u64));
    group.bench_function("no_simd", |b| {
        b.iter(|| {
            black_box(no_simd(arr, count));
        })
    });
    group.bench_function("no_simd_unroll4", |b| {
        b.iter(|| {
            black_box(no_simd_unroll4(arr, count));
        })
    });
    group.bench_function("auto_simd", |b| {
        b.iter(|| {
            black_box(auto_simd(arr, count));
        })
    });
    group.bench_function("std_128", |b| {
        b.iter(|| {
            black_box(std_simd_128(arr, count));
        })
    });
    group.bench_function("std_256", |b| {
        b.iter(|| {
            black_box(std_simd_256(arr, count));
        })
    });
    group.bench_function("avx_256", |b| {
        b.iter(|| {
            black_box(unsafe { avx_simd_256(arr, count) });
        })
    });
    group.bench_function("std_256_unroll4", |b| {
        b.iter(|| {
            black_box(std_simd_256_unroll4(arr, count));
        })
    });
    group.bench_function("std_256_unroll4x2", |b| {
        b.iter(|| {
            black_box(std_simd_256_unroll4x2(arr, count));
        })
    });
    group.bench_function("std_256_unroll4x4", |b| {
        b.iter(|| {
            black_box(std_simd_256_unroll4x4(arr, count));
        })
    });

    unsafe {
        std::alloc::dealloc(arr as *mut u8, layout);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = simd_sum
);
criterion_main!(benches);
