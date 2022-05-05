use criterion::black_box;

pub fn div_u64() {
    repeat100!(black_box(black_box(64 as u64) / black_box(3 as u64)));
}

pub fn div_f64() {
    repeat100!(black_box(black_box(64 as f64) / black_box(3 as f64)));
}