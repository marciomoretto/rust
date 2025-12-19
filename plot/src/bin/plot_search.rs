use plot::plot::{plot_from_csv, PlotScale};

const OUTPUT_DIR: &str = "../IAA/search/benches/output";

fn main() {
    let workloads = ["hit_middle", "miss_low", "miss_high"];

    let csv = format!("{OUTPUT_DIR}/search.csv");

    // 1) log–log → amostra geométrica (boa para x = log n)
    plot_from_csv(&csv, &workloads, OUTPUT_DIR, true, PlotScale::LogLog)
        .expect("erro ao gerar gráficos log–log");
}
