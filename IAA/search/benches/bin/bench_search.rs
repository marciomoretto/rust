use std::error::Error;

use search::algorithms::IMPLEMENTATIONS;
use search::bench_utils::core::time_it_ns_per_op;
use search::bench_utils::workloads::{choose_x, make_sorted_vec, WORKLOADS};

fn main() -> Result<(), Box<dyn Error>> {
    // Cabeçalho CSV (compatível com o plot)
    println!("implementation;workload;n;time_total;time_per_op");

    const SEED: u64 = 12345;
    const REPS: u64 = (1usize << 16) as u64;

    // Parâmetros da amostra (em escala log)
    let n_points = 32;
    let start_n: f64 = (1usize << 8) as f64;
    let end_n:   f64 = (1usize << 24) as f64;

    let log_start: f64 = start_n.log10();
    let log_end:   f64 = end_n.log10();

    // Amostra geométrica: n igualmente espaçados em log10(n)
    let ns: Vec<usize> = (0..n_points)
        .map(|i| {
            let t = i as f64 / (n_points - 1) as f64;
            let log_n: f64 = log_start + t * (log_end - log_start);
            10f64.powf(log_n).round() as usize
        })
        .collect();

    for &workload in WORKLOADS {
        for &n in &ns {
            let a = make_sorted_vec(n);

            for imp in IMPLEMENTATIONS {
                let x = choose_x(&a, workload, SEED);

                let ns_per_op = time_it_ns_per_op(REPS, || {
                    let found = (imp.f)(&a, x);
                    std::hint::black_box(found);
                });

                let time_total = ns_per_op * (REPS as f64);

                println!(
                    "{};{};{};{:.6};{:.6}",
                    imp.name,
                    workload.name(),
                    n,
                    time_total,
                    ns_per_op
                );
            }
        }
    }

    Ok(())
}
