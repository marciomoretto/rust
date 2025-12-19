use sorting::algorithms::*;
use sorting::benches::*;

/// Gera n_points valores igualmente espaçados entre min e max (inclusive)
fn linspace_usize(min: usize, max: usize, n_points: usize) -> Vec<usize> {
    assert!(n_points >= 2);

    let step = (max - min) / (n_points - 1);

    (0..n_points)
        .map(|i| min + i * step)
        .collect()
}

fn main() {
    // Cabeçalho compatível com time_it_k
    println!("implementation;workload;n;k;time_total;time_per_op");

    let n_points = 20;

    let min = 1usize << 20; // 2^20
    let max = 1usize << 31; // 2^26

    // ✅ seis pontos LINEARES em valor
    let ns = linspace_usize(min, max, n_points);
    let ks = linspace_usize(min, max, n_points);

    for &n in &ns {
        bench_linear_n::<RadixSort>(n);
    }

    // Varre o grid completo (n, k)
    for &k in &ks {
        for &n in &ns {
            bench_linear_nk::<CountingSort>(n, k);
        }
    }
}
