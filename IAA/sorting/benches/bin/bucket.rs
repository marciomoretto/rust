use sorting::algorithms::*;
use sorting::benches::*;

fn main() {
    println!("implementation;workload;n;time_total;time_per_op");

    let n_points = 20;

    let start = 1_000.0_f64;
    let end   = 200_000.0_f64;

    let log_start = start.log10();
    let log_end   = end.log10();

    let ns: Vec<usize> = (0..n_points)
        .map(|i| {
            let t = i as f64 / (n_points - 1) as f64;
            10f64.powf(log_start + t * (log_end - log_start)) as usize
        })
        .collect();

    for &n in &ns {
        // BucketSort: caso esperado (uniforme)
        bench_bucket_uniform::<BucketSort>(n);

        // BucketSort: pior caso estrutural
        bench_bucket_worst::<BucketSort>(n);

        // Opcional: pior caso + desordem interna
        bench_bucket_worst_shuffled::<BucketSort>(n);
    }
}
