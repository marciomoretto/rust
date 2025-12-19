use plot::plot::{plot_from_csv, PlotScale};

const OUTPUT_DIR: &str = "../IAA/primes/benches/output";

fn main() {
    let workloads = ["prime_worst_case_bits"];

    let csv_bits = format!("{OUTPUT_DIR}/primes-bits.csv");
    let csv = format!("{OUTPUT_DIR}/primes.csv");

    // 1) log–log → amostra geométrica (boa para x = log n)
    plot_from_csv(&csv, &workloads, OUTPUT_DIR, true, PlotScale::LogLog)
        .expect("erro ao gerar gráficos log–log");

    // 2) log–lin → amostra linear (boa para x = n)
    plot_from_csv(&csv_bits, &workloads, OUTPUT_DIR, true, PlotScale::LogLin)
        .expect("erro ao gerar gráficos log-lin");
}
