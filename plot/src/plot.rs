use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::prelude::*;
use plotters::style::RGBColor;

#[derive(Debug, Deserialize)]
struct Record {
    implementation: String,
    workload: String,
    n: usize,
    time_total: f64,
    time_per_op: f64,
}

/// Lê o CSV, separa por *workload*, gera um PNG por workload e
/// salva os coeficientes de regressão em:
///     output_dir/regression.csv
///
/// Se `use_time_per_op` for `true`, usa `time_per_op` na regressão.
/// Caso contrário, usa `time_total`.
pub fn plot_from_csv(
    csv_path: &str,
    workloads: &[&str],
    output_dir: &str,
    use_time_per_op: bool,
) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(output_dir)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)?;

    let mut records: Vec<Record> = Vec::new();
    for result in rdr.deserialize::<Record>() {
        let rec = result?;
        records.push(rec);
    }

    // Agrupa por workload
    let mut by_workload: HashMap<String, Vec<Record>> = HashMap::new();
    for rec in records {
        by_workload
            .entry(rec.workload.clone())
            .or_default()
            .push(rec);
    }

    // Arquivo CSV com coeficientes de regressão
    let mut reg_file = File::create(Path::new(output_dir).join("regression.csv"))?;
    writeln!(&mut reg_file, "workload;implementation;alpha;C")?;

    // Plota cada workload pedido
    for &workload in workloads {
        if let Some(recs) = by_workload.get(workload) {
            if !recs.is_empty() {
                plot_workload(
                    workload,
                    recs,
                    output_dir,
                    &mut reg_file,
                    use_time_per_op,
                )?;
            }
        }
    }

    Ok(())
}

fn plot_workload(
    workload: &str,
    recs: &[Record],
    output_dir: &str,
    reg_out: &mut dyn Write,
    use_time_per_op: bool,
) -> Result<(), Box<dyn Error>> {
    // Converte para log-log imediatamente
    let mut by_impl_log: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

    let mut min_lx = f64::INFINITY;
    let mut max_lx = f64::NEG_INFINITY;
    let mut min_ly = f64::INFINITY;
    let mut max_ly = f64::NEG_INFINITY;

    for rec in recs {
        if rec.n == 0 {
            continue;
        }

        let y_raw = if use_time_per_op {
            rec.time_per_op
        } else {
            rec.time_total
        };

        if y_raw <= 0.0 {
            continue;
        }

        let lx = (rec.n as f64).log10();
        let ly = y_raw.log10();

        by_impl_log
            .entry(rec.implementation.clone())
            .or_default()
            .push((lx, ly));

        min_lx = min_lx.min(lx);
        max_lx = max_lx.max(lx);
        min_ly = min_ly.min(ly);
        max_ly = max_ly.max(ly);
    }

    if !min_lx.is_finite() || !min_ly.is_finite() {
        return Ok(()); // nada pra plotar
    }

    // Área do gráfico
    let filename = format!("{}/{}.png", output_dir, workload);
    let root = BitMapBackend::new(&filename, (900, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    let eixo_y_label = if use_time_per_op {
        "log10(tempo por operação)"
    } else {
        "log10(tempo total)"
    };

    // Sistema de coordenadas já em log-log
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{workload} — log-log + regressão"),
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min_lx..max_lx, min_ly..max_ly)?;

    chart
        .configure_mesh()
        .x_desc("log10(n)")
        .y_desc(eixo_y_label)
        .label_style(("sans-serif", 15))
        .draw()?;

    // Paleta de cores
    let colors = [
        RGBColor(76, 114, 176),
        RGBColor(221, 132, 82),
        RGBColor(85, 168, 104),
        RGBColor(196, 78, 82),
        RGBColor(129, 114, 178),
        RGBColor(147, 120, 96),
        RGBColor(140, 140, 140),
    ];

    // Plota cada implementação
    for (idx, (impl_name, mut pts)) in by_impl_log.into_iter().enumerate() {
        if pts.len() < 2 {
            continue;
        }
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();

        let (slope, intercept) = linear_regression(&xs, &ys);
        let c = 10f64.powf(intercept);
        let color = colors[idx % colors.len()];

        // Escreve coeficientes no CSV
        writeln!(reg_out, "{workload};{impl_name};{slope:.4};{c:.4e}")?;

        // Pontos
        chart.draw_series(
            pts.iter()
                .map(|(lx, ly)| Circle::new((*lx, *ly), 3, color.filled())),
        )?;

        // Reta
        let x1 = xs.first().copied().unwrap();
        let x2 = xs.last().copied().unwrap();
        let y1 = slope * x1 + intercept;
        let y2 = slope * x2 + intercept;

        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x1, y1), (x2, y2)],
                color.stroke_width(1),
            )))?
            .label(impl_name)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2))
            });
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 15))
        .draw()?;

    Ok(())
}

/// Regressão linear: y = a x + b
/// Usando fórmula centrada na média (menos propensa a overflow)
pub fn linear_regression(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len();
    if n == 0 {
        return (0.0, 0.0);
    }

    let n_f = n as f64;
    let mean_x = xs.iter().sum::<f64>() / n_f;
    let mean_y = ys.iter().sum::<f64>() / n_f;

    let mut num = 0.0;
    let mut den = 0.0;

    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let dx = x - mean_x;
        num += dx * (y - mean_y);
        den += dx * dx;
    }

    if den == 0.0 {
        return (0.0, mean_y);
    }

    let a = num / den;
    let b = mean_y - a * mean_x;
    (a, b)
}
