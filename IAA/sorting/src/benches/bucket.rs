use rand::prelude::*;
use std::hint::black_box;

use ordered_float::OrderedFloat;

use super::core::time_it;
use crate::algorithms::Sorter;

fn uniform01_vec(n: usize) -> Vec<OrderedFloat<f64>> {
    let mut rng = thread_rng();
    (0..n).map(|_| OrderedFloat(rng.gen::<f64>())).collect()
}

fn worst_bucket_vec(n: usize) -> Vec<OrderedFloat<f64>> {
    let nn = (n as f64) * (n as f64);
    (0..n).map(|i| OrderedFloat((i as f64) / nn)).collect()
}

fn worst_bucket_shuffled_vec(n: usize) -> Vec<OrderedFloat<f64>> {
    let mut v = worst_bucket_vec(n);
    v.shuffle(&mut thread_rng());
    v
}

pub fn bench_bucket_uniform<S>(n: usize)
where
    S: Sorter<OrderedFloat<f64>>,
{
    time_it(S::name(), "bucket_uniform", n, n, || {
        let mut v = black_box(uniform01_vec(n));
        S::sort(black_box(v.as_mut_slice()));
        black_box(&v);
    });
}

pub fn bench_bucket_worst<S>(n: usize)
where
    S: Sorter<OrderedFloat<f64>>,
{
    time_it(S::name(), "bucket_worst", n, n, || {
        let mut v = black_box(worst_bucket_vec(n));
        S::sort(black_box(v.as_mut_slice()));
        black_box(&v);
    });
}

pub fn bench_bucket_worst_shuffled<S>(n: usize)
where
    S: Sorter<OrderedFloat<f64>>,
{
    time_it(S::name(), "bucket_worst_shuffled", n, n, || {
        let mut v = black_box(worst_bucket_shuffled_vec(n));
        S::sort(black_box(v.as_mut_slice()));
        black_box(&v);
    });
}
