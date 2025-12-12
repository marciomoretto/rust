pub mod core;
pub mod sort;

pub use sort::{
    bench_sort_random,
    bench_sort_sorted,
    bench_sort_reversed,
    bench_sort_almost_sorted,
};
