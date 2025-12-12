use plot::plot_from_csv;

const OUTPUT_DIR: &str = "../IAA/duplicates/benches/output";

fn main() {
    let workloads = [
        "dup_no_repeat",
    ];

    let csv_path = format!("{OUTPUT_DIR}/duplicates.csv");

    // último argumento: use_time_per_op = false  → usa time_total
    if let Err(e) = plot_from_csv(&csv_path, &workloads, OUTPUT_DIR, false) {
        eprintln!("erro ao gerar gráficos de duplicates: {e}");
        std::process::exit(1);
    }
}
