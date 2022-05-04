#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(stdarch)]

use criterion::black_box;

use core::arch::x86_64::_mm_load_si128;
use std::arch::x86_64::{
    _mm256_add_epi32, _mm256_castsi256_si128, _mm256_extracti128_si256, _mm256_load_si256,
    _mm256_setzero_si256, _mm_add_epi32, _mm_cvtsi128_si32, _mm_shuffle_epi32, _mm_unpackhi_epi64,
    _MM_SHUFFLE,
};
use std::simd::{i32x4, i32x8};

pub fn no_simd(arr: *const i32, count: usize) -> i32 {
    let mut sum = 0;
    for i in 0..count {
        sum += black_box(unsafe { arr.add(i).read() });
    }
    sum
}

pub fn no_simd_unroll4(arr: *const i32, count: usize) -> i32 {
    let mut sum = 0;
    for i in (0..count).step_by(4) {
        let mut local_sum = 0;
        local_sum += black_box(unsafe { arr.add(i).read() });
        local_sum += black_box(unsafe { arr.add(i + 1).read() });
        local_sum += black_box(unsafe { arr.add(i + 2).read() });
        local_sum += black_box(unsafe { arr.add(i + 3).read() });
        sum += local_sum;
    }
    sum
}

pub fn auto_simd(arr: *const i32, count: usize) -> i32 {
    let mut sum = 0;
    for i in 0..count {
        sum += unsafe { arr.add(i).read() };
    }
    sum
}

pub fn std_simd_128(arr: *const i32, count: usize) -> i32 {
    let mut sum = i32x4::splat(0);
    for i in (0..count).step_by(4) {
        let v = i32x4::from(unsafe { _mm_load_si128(std::mem::transmute(arr.add(i))) });
        sum += v;
    }
    let sum = sum.reduce_sum();
    sum
}

pub fn std_simd_256(arr: *const i32, count: usize) -> i32 {
    let mut sum = i32x8::splat(0);
    for i in (0..count).step_by(8) {
        let v = i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i))) });
        sum += v;
    }
    let sum = sum.reduce_sum();
    sum
}

pub unsafe fn avx_simd_256(arr: *const i32, count: usize) -> i32 {
    let mut sum = _mm256_setzero_si256();
    for i in (0..count).step_by(8) {
        let v = _mm256_load_si256(std::mem::transmute(arr.add(i)));
        sum = _mm256_add_epi32(sum, v);
    }
    let tmp = _mm256_extracti128_si256::<1>(sum);
    let sum = _mm_add_epi32(_mm256_castsi256_si128(sum), tmp);
    let high = _mm_unpackhi_epi64(sum, sum);
    let sum = _mm_add_epi32(sum, high);
    let high = _mm_shuffle_epi32::<{ _MM_SHUFFLE(2, 3, 0, 1) }>(sum);
    let sum = _mm_add_epi32(sum, high);
    let sum = _mm_cvtsi128_si32(sum);
    sum
}

pub fn std_simd_256_unroll4(arr: *const i32, count: usize) -> i32 {
    let mut sum = i32x8::splat(0);
    for i in (0..count).step_by(32) {
        let mut local_sum = i32x8::splat(0);
        local_sum += i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i))) });
        local_sum += i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 8))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 16))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 24))) });
        sum += local_sum;
    }
    let sum = sum.reduce_sum();
    sum
}

pub fn std_simd_256_unroll8(arr: *const i32, count: usize) -> i32 {
    let mut sum = i32x8::splat(0);
    for i in (0..count).step_by(64) {
        let mut local_sum = i32x8::splat(0);
        local_sum += i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i))) });
        local_sum += i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 8))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 16))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 24))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 32))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 40))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 48))) });
        local_sum +=
            i32x8::from(unsafe { _mm256_load_si256(std::mem::transmute(arr.add(i + 56))) });
        sum += local_sum;
    }
    let sum = sum.reduce_sum();
    sum
}

#[test]
fn test_avx_simd_256() {
    use std::alloc::Layout;
    let count = 32;
    let layout = Layout::from_size_align(count * std::mem::size_of::<i32>(), 32).unwrap();
    let arr = unsafe { std::alloc::alloc(layout) as *mut i32 };
    let mut expected_sum = 0;
    for i in 0..count {
        expected_sum += i as i32;
        unsafe {
            *arr.add(i) = i as i32;
        }
    }
    let sum = unsafe { avx_simd_256(arr, count) };
    assert_eq!(sum, expected_sum);
    unsafe { std::alloc::dealloc(arr as *mut u8, layout) };
}

pub fn div_u64() {
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
    black_box(black_box(64 as u64) / black_box(3 as u64));
}

pub fn div_f64() {
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
    black_box(black_box(64 as f64) / black_box(3 as f64));
}
