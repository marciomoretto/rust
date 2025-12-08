use std::time::Instant;

/// Mede o tempo de um workload e imprime:
/// implementação;workload;n;tempo_total;tempo_por_op
pub fn time_it<F>(impl_name: &str, workload: &str, n: usize, ops: usize, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();

    let time = elapsed.as_secs_f64();
    let time_per_op = time / ops as f64;

    println!(
        "{impl_name};{workload};{n};{time};{time_per_op}"
    );
}
