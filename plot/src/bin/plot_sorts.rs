use plot::plot_from_csv;

const OUTPUT_DIR: &str = "../IAA/sorting/benches/output";

fn main() {
    let workloads = [
        "sort_random",
        "sort_sorted",
        "sort_reversed",
        "sort_almost_sorted",
    ];

    let csv_path = format!("{OUTPUT_DIR}/sort.csv");

    if let Err(e) = plot_from_csv(&csv_path, &workloads, OUTPUT_DIR, true) {
        eprintln!("erro ao gerar gráficos de sorting: {e}");
        std::process::exit(1);
    }
}
