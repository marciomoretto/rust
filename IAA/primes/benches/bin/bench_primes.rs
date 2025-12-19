use ::primes::algorithms::*;
use ::primes::bench_utils::*;

fn main() {
    println!("implementation;workload;n;time_total;time_per_op");

    let n_points = 32; // bits 8..24 inclusive => 17 pontos
    let reps = 64;

    // “n” aqui vai ser bits (8..24) em escala linear mesmo, mas mantendo o mesmo loop
    // do seu exemplo para ficar idêntico.
    // Se você quiser logspace de bits (não faz muito sentido), eu adapto.
    let start = 8.0_f64;
    let end   = 63.0_f64;

    let ns: Vec<usize> = (0..n_points)
        .map(|i| {
            let t = i as f64 / (n_points - 1) as f64;
            (start + t * (end - start)).round() as usize
        })
        .collect();

    for bits in ns {
        bench_prime_worst_case::<TrialSqrt>(bits as u32, reps);
        // se depois você criar outra implementação:
        // bench_prime_worst_case::<TrialSqrtEvenSkipping>(bits as u32, reps);
    }
}

