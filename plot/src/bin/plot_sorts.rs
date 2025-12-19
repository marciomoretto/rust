use plot::plot::{plot_from_csv, PlotScale};
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

const OUTPUT_DIR: &str = "../IAA/sorting/benches/output";

fn main() {
    let workloads = [
        "sort_random",
        "sort_sorted",
        "sort_reversed",
        "sort_almost_sorted",
        "sort_linear_n",
        "bucket_uniform",
        "bucket_worst",
        "bucket_worst_shuffled",
    ];

    let csv_path = format!("{OUTPUT_DIR}/sort.csv");

    if let Err(e) = run(&csv_path, &workloads) {
        eprintln!("erro ao gerar gráficos de sorting: {e}");
        std::process::exit(1);
    }
}

fn run(csv_path: &str, workloads: &[&str]) -> Result<(), Box<dyn Error>> {
    let (list, selected, scale) = parse_args();

    if list {
        let algos = list_implementations(csv_path)?;
        for a in algos {
            println!("{a}");
        }
        return Ok(());
    }

    let csv_to_plot = if selected.is_empty() {
        csv_path.to_string()
    } else {
        let out = format!("{OUTPUT_DIR}/sort.filtered.csv");
        filter_csv_by_implementation(csv_path, &out, &selected)?;
        out
    };

    plot_from_csv(&csv_to_plot, workloads, OUTPUT_DIR, false, scale)?;
    Ok(())
}

fn parse_args() -> (bool, HashSet<String>, PlotScale) {
    let mut list = false;
    let mut selected: HashSet<String> = HashSet::new();
    let mut scale = PlotScale::LogLog; // default

    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--list" => list = true,
            "--algos" => {
                if let Some(v) = it.next() {
                    for s in v.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()) {
                        selected.insert(s.to_string());
                    }
                }
            }
            "--scale" => {
                if let Some(v) = it.next() {
                    scale = match v.as_str() {
                        "linlin" => PlotScale::LinLin,
                        "loglog" => PlotScale::LogLog,
                        "linlog" => PlotScale::LinLog,
                        "loglin" => PlotScale::LogLin,
                        _ => scale,
                    };
                }
            }
            _ => {}
        }
    }

    (list, selected, scale)
}


fn list_implementations(csv_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)?;

    let mut set = HashSet::new();
    for rec in rdr.records() {
        let rec = rec?;
        // colunas: implementation;workload;n;time_total;time_per_op
        if let Some(impl_name) = rec.get(0) {
            set.insert(impl_name.to_string());
        }
    }

    let mut v: Vec<_> = set.into_iter().collect();
    v.sort();
    Ok(v)
}

fn filter_csv_by_implementation(
    in_path: &str,
    out_path: &str,
    allowed: &HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    // garante a pasta de saída
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(in_path)?;

    let headers = rdr.headers()?.clone();

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(out_path)?;

    wtr.write_record(&headers)?;

    for rec in rdr.records() {
        let rec = rec?;
        let impl_name = rec.get(0).unwrap_or("");
        if allowed.contains(impl_name) {
            wtr.write_record(&rec)?;
        }
    }

    wtr.flush()?;
    Ok(())
}
