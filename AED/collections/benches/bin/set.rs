use collections::set::*;
use collections::benches::*;

fn main() {
    println!("implementation;workload;n;time_total;time_per_op");

    let n_points = 20;

    let start = 1_000.0_f64;
    let end   = 200_000.0_f64;

    let log_start = start.log10();
    let log_end   = end.log10();

    let ns: Vec<usize> = (0..n_points)
        .map(|i| {
            let t = i as f64 / (n_points - 1) as f64; // 0 → 1
            10f64.powf(log_start + t * (log_end - log_start)) as usize
        })
        .collect();

    for &n in ns.iter() {
        bench_insert_distinct::<ListSet<i32>>(n);
        bench_lookup_hit::<ListSet<i32>>(n);
        bench_lookup_miss::<ListSet<i32>>(n); 
        bench_insert_sorted::<ListSet<i32>>(n);

        bench_insert_distinct::<BstSet<i32>>(n);
        bench_lookup_hit::<BstSet<i32>>(n);
        bench_lookup_miss::<BstSet<i32>>(n);
        bench_insert_sorted::<BstSet<i32>>(n);

        bench_insert_distinct::<BstAvlSet<i32>>(n);
        bench_lookup_hit::<BstAvlSet<i32>>(n);
        bench_lookup_miss::<BstAvlSet<i32>>(n);
        bench_insert_sorted::<BstAvlSet<i32>>(n);

        bench_insert_distinct::<BstRBSet<i32>>(n);
        bench_lookup_hit::<BstRBSet<i32>>(n);
        bench_lookup_miss::<BstRBSet<i32>>(n);
        bench_insert_sorted::<BstRBSet<i32>>(n);

        bench_insert_distinct::<HashChainingSet<i32>>(n);
        bench_lookup_hit::<HashChainingSet<i32>>(n);
        bench_lookup_miss::<HashChainingSet<i32>>(n);
        bench_insert_sorted::<HashChainingSet<i32>>(n);

        bench_insert_distinct::<HashProbingSet<i32>>(n);
        bench_lookup_hit::<HashProbingSet<i32>>(n);
        bench_lookup_miss::<HashProbingSet<i32>>(n); 
        bench_insert_sorted::<HashProbingSet<i32>>(n);
    }
}
