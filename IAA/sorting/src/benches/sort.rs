use rand::prelude::*;
use super::core::time_it;
use crate::algorithms::Sorter;

fn random_vec(n: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..n as i32).collect();
    v.shuffle(&mut thread_rng());
    v
}

fn sorted_vec(n: usize) -> Vec<i32> {
    (0..n as i32).collect()
}

fn reversed_vec(n: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..n as i32).collect();
    v.reverse();
    v
}

fn almost_sorted_vec(n: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..n as i32).collect();
    let swaps = (n / 20).max(1);
    for _ in 0..swaps {
        let i = thread_rng().gen_range(0..n);
        let j = thread_rng().gen_range(0..n);
        v.swap(i, j);
    }
    v
}

// Cada função é genérica no algoritmo, igual aos benches de Set.

pub fn bench_sort_random<S>(n: usize)
where
    S: Sorter<i32>,
{
    time_it(S::name(), "sort_random", n, n, || {
        let mut v = random_vec(n);
        S::sort(&mut v);
    });
}

pub fn bench_sort_sorted<S>(n: usize)
where
    S: Sorter<i32>,
{
    time_it(S::name(), "sort_sorted", n, n, || {
        let mut v = sorted_vec(n);
        S::sort(&mut v);
    });
}

pub fn bench_sort_reversed<S>(n: usize)
where
    S: Sorter<i32>,
{
    time_it(S::name(), "sort_reversed", n, n, || {
        let mut v = reversed_vec(n);
        S::sort(&mut v);
    });
}

pub fn bench_sort_almost_sorted<S>(n: usize)
where
    S: Sorter<i32>,
{
    time_it(S::name(), "sort_almost_sorted", n, n, || {
        let mut v = almost_sorted_vec(n);
        S::sort(&mut v);
    });
}
