pub mod core;
pub mod sort;
pub mod linear;
pub mod bucket;

pub use sort::{
    bench_sort_random,
    bench_sort_sorted,
    bench_sort_reversed,
    bench_sort_almost_sorted,
};

pub use linear::{
    bench_linear_nk,
    bench_linear_n,
};

pub use bucket::{
    bench_bucket_uniform,
    bench_bucket_worst,
    bench_bucket_worst_shuffled,
};