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
            let t = i as f64 / (n_points - 1) as f64; // 0 → 1
            10f64.powf(log_start + t * (log_end - log_start)) as usize
        })
        .collect();

    for &n in &ns {
        // Bubble
        bench_sort_random::<Bubble>(n);
        bench_sort_sorted::<Bubble>(n);
        bench_sort_reversed::<Bubble>(n);
        bench_sort_almost_sorted::<Bubble>(n);

        // Insertion
        bench_sort_random::<Insertion>(n);
        bench_sort_sorted::<Insertion>(n);
        bench_sort_reversed::<Insertion>(n);
        bench_sort_almost_sorted::<Insertion>(n);

        // Selection
        bench_sort_random::<Selection>(n);
        bench_sort_sorted::<Selection>(n);
        bench_sort_reversed::<Selection>(n);
        bench_sort_almost_sorted::<Selection>(n);

        // Merge sort (top-down)
        bench_sort_random::<MergeSort>(n);
        bench_sort_sorted::<MergeSort>(n);
        bench_sort_reversed::<MergeSort>(n);
        bench_sort_almost_sorted::<MergeSort>(n);

        // QuickSort
        bench_sort_random::<QuickSort>(n);
        bench_sort_sorted::<QuickSort>(n);
        bench_sort_reversed::<QuickSort>(n);
        bench_sort_almost_sorted::<QuickSort>(n);

        // HeapSort
        bench_sort_random::<HeapSort>(n);
        bench_sort_sorted::<HeapSort>(n);
        bench_sort_reversed::<HeapSort>(n);
        bench_sort_almost_sorted::<HeapSort>(n);

        // Rust std
        bench_sort_random::<RustStd>(n);
        bench_sort_sorted::<RustStd>(n);
        bench_sort_reversed::<RustStd>(n);
        bench_sort_almost_sorted::<RustStd>(n);
    }
}
