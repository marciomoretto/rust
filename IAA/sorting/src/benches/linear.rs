use rand::prelude::*;
use std::hint::black_box;

use super::core::time_it_k;
use crate::algorithms::Sorter;

/// Gera n inteiros i32 uniformes em todo o domínio
fn random_vec_i32(n: usize) -> Vec<i32> {
    if n == 0 {
        return vec![];
    }

    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen::<i32>()).collect()
}

/// Gera um vetor com n inteiros uniformes em [0, k).
/// Aqui "k" é o parâmetro que controla o domínio de chaves.
/// Para counting sort com min=0, vale R = k.
fn random_vec_domain(n: usize, k: usize) -> Vec<i32> {
    if n == 0 {
        return vec![];
    }
    if k <= 1 {
        return vec![0; n];
    }

    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(0..k) as i32).collect()
}

pub fn bench_linear_n<S>(n: usize)
where
    S: Sorter<i32>,
{
    time_it_k(S::name(), "sort_linear_n", n, 0, || {
        let mut v = black_box(random_vec_i32(n)); // uniform i32
        S::sort(black_box(v.as_mut_slice()));
        black_box(&v);
    });
}

pub fn bench_linear_nk<S>(n: usize, k: usize)
where
    S: Sorter<i32>,
{
    time_it_k(S::name(), "sort_linear_nk", n, k, || {
        let mut v = black_box(random_vec_domain(n, k));
        S::sort(black_box(v.as_mut_slice()));
        black_box(&v);
    });
}
