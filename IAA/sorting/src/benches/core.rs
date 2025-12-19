use std::time::Instant;

/// CSV antigo (não muda):
/// implementation;workload;n;time_total;time_per_op
pub fn time_it<F>(impl_name: &str, workload: &str, n: usize, ops: usize, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();

    let time_total = elapsed.as_secs_f64();
    let time_per_op = time_total / ops as f64;

    println!("{impl_name};{workload};{n};{time_total};{time_per_op}");
}

/// CSV novo (só para linear):
/// implementation;workload;n;k;time_total;time_per_n
pub fn time_it_k<F>(impl_name: &str, workload: &str, n: usize, k: usize, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();

    let time_total = elapsed.as_secs_f64();
    let time_per_n = time_total / n as f64;

    println!("{impl_name};{workload};{n};{k};{time_total};{time_per_n}");
}
