use plot::plot_from_csv;

const OUTPUT_DIR: &str = "../AED/collections/benches/output";

fn main() {
    let workloads = [
        "insert_distinct",
        "lookup_hit",
        "lookup_miss",
        "insert_sorted",
    ];

    let csv_path = format!("{OUTPUT_DIR}/set.csv");

    // último argumento: use_time_per_op = true
    if let Err(e) = plot_from_csv(&csv_path, &workloads, OUTPUT_DIR, true) {
        eprintln!("erro ao gerar gráficos de sets: {e}");
        std::process::exit(1);
    }
}
