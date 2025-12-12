use super::core::time_it;
use crate::algorithms::DuplicateChecker;
use std::hint::black_box;

fn vec_without_duplicates(n: usize) -> Vec<usize> {
    (0..n).collect()
}

pub fn bench_duplicates_worst_case<D>(n: usize, reps: usize)
where
    D: DuplicateChecker<usize>,
{
    // Gera o vetor e passa por black_box para o compilador não
    // “entender demais” sobre ele.
    let v = vec_without_duplicates(n);
    let v = black_box(v);

    time_it(D::name(), "dup_no_repeat", n, n * reps, || {
        for _ in 0..reps {
            // Garante que a chamada não seja podada
            let res = D::has_duplicate(&v);
            black_box(res);
        }
    });
}
