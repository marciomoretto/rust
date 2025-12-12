use ::duplicates::algorithms::*;
use ::duplicates::benches::*;

fn main() {
    println!("implementation;workload;n;time_total;time_per_op");

    let n_points = 20;
    let reps = 1;
    let start = 1_000.0_f64;
    let end   = 500_000.0_f64;

    let log_start = start.log10();
    let log_end   = end.log10();

    let ns: Vec<usize> = (0..n_points)
        .map(|i| {
            let t = i as f64 / (n_points - 1) as f64;
            let log_n = log_start + t * (log_end - log_start);
            10f64.powf(log_n).round() as usize
        })
        .collect();

    for n in ns {
        bench_duplicates_worst_case::<Naive>(n, reps);
        bench_duplicates_worst_case::<LinearSeen>(n, reps);
    }
}
