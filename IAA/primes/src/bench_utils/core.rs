use std::time::Instant;

/// Mede o tempo de um workload e imprime:
/// implementation;workload;n;time_total;time_per_op
///
/// - `impl_name`: nome da implementação (ex: "TrialSqrt")
/// - `workload`: nome do workload (ex: "prime_worst_case_bits")
/// - `n`: tamanho do input (no seu caso pode ser `bits` ou o próprio número)
/// - `ops`: número de operações (repetições) executadas dentro de `f`
/// - `f`: closure que executa exatamente `ops` operações e “consome” o resultado
pub fn time_it<F>(impl_name: &str, workload: &str, n: u64, ops: u64, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();

    let time_total = elapsed.as_secs_f64();
    let time_per_op = time_total / (ops as f64);

    println!("{impl_name};{workload};{n};{time_total};{time_per_op}");
}

