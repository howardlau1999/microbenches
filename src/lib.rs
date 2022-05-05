#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(stdarch)]

macro_rules! repeat2 {
    ($x:expr) => {
        $x;
        $x;
    };
}

macro_rules! repeat4 {
    ($x:expr) => {
        repeat2!($x);
        repeat2!($x);
    };
}

macro_rules! repeat5 {
    ($x:expr) => {
        repeat2!($x);
        repeat2!($x);
        $x;
    };
}

macro_rules! repeat100 {
    ($x:expr) => {
        repeat4!(repeat5!(repeat5!($x)));
    };
}

pub mod simd_sum;
pub mod mlp;
pub mod div;

