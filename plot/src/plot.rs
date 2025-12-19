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

/// Escalas suportadas (mantém LOG-LOG como padrão):
///
/// - LogLog: x = log10(n), y = log10(tempo)
/// - LogLin: x = n,        y = log10(tempo)
/// - LinLog: x = log10(n), y = tempo
#[derive(Clone, Copy, Debug)]
pub enum PlotScale {
    LogLog,
    LogLin,
    LinLog,
    LinLin,
}

impl PlotScale {
    fn tag(self) -> &'static str {
        match self {
            PlotScale::LogLog => "loglog",
            PlotScale::LogLin => "loglin",
            PlotScale::LinLog => "linlog",
            PlotScale::LinLin => "linlin",
        }
    }

    fn title(self) -> &'static str {
        match self {
            PlotScale::LogLog => "log–log",
            PlotScale::LogLin => "log–lin",
            PlotScale::LinLog => "lin–log",
            PlotScale::LinLin => "lin–lin",
        }
    }

    fn x_label(self) -> &'static str {
        match self {
            PlotScale::LogLog | PlotScale::LinLog => "log10(n)",
            PlotScale::LogLin | PlotScale::LinLin => "n (×10⁶)",
        }
    }

      fn y_label(self, use_time_per_op: bool) -> &'static str {
        match self {
            // y linear
            PlotScale::LinLog | PlotScale::LinLin => {
                if use_time_per_op { "tempo por operação" } else { "tempo total" }
            }
            // y log
            PlotScale::LogLog | PlotScale::LogLin => {
                if use_time_per_op { "log10(tempo por operação)" } else { "log10(tempo total)" }
            }
        }
    }
}

fn project_point(n: usize, time: f64, scale: PlotScale) -> Option<(f64, f64)> {
    if n == 0 || time <= 0.0 {
        return None;
    }

    let n_f = n as f64;

    let (x, y) = match scale {
        PlotScale::LogLog => (n_f.log10(), time.log10()),
        PlotScale::LogLin => (n_f / 1e6, time.log10()),
        PlotScale::LinLog => (n_f.log10(), time),
        PlotScale::LinLin => (n_f / 1e6, time),
    };

    Some((x, y))
}

/// R² da regressão y = a x + b
pub fn r2_score(xs: &[f64], ys: &[f64], a: f64, b: f64) -> f64 {
    if xs.len() != ys.len() || xs.len() < 2 {
        return f64::NAN;
    }

    let mean_y = ys.iter().sum::<f64>() / (ys.len() as f64);

    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;

    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let y_hat = a * x + b;
        let dy = y - y_hat;
        ss_res += dy * dy;

        let dt = y - mean_y;
        ss_tot += dt * dt;
    }

    if ss_tot == 0.0 {
        1.0
    } else {
        1.0 - (ss_res / ss_tot)
    }
}

/// Lê o CSV, separa por workload, gera PNGs e
/// salva os coeficientes de regressão.
///
/// Para não sobrescrever, salva:
///   regression_<tag>.csv
///
/// CSV gerado:
///   workload;implementation;alpha;C;R2
///
/// Interpretação:
/// - Se Y está em log10 (LogLog/LogLin):    log10(t) = alpha * x + log10(C)  => C = 10^intercept
/// - Se Y é linear (LinLog):               t = alpha * log10(n) + C         => C = intercept
pub fn plot_from_csv(
    csv_path: &str,
    workloads: &[&str],
    output_dir: &str,
    use_time_per_op: bool,
    scale: PlotScale,
) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(output_dir)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)?;

    let mut records: Vec<Record> = Vec::new();
    for result in rdr.deserialize::<Record>() {
        records.push(result?);
    }

    // Agrupa por workload
    let mut by_workload: HashMap<String, Vec<Record>> = HashMap::new();
    for rec in records {
        by_workload
            .entry(rec.workload.clone())
            .or_default()
            .push(rec);
    }

    // CSV de regressão (separado por escala)
    let reg_path = Path::new(output_dir).join(format!("regression_{}.csv", scale.tag()));
    let mut reg_file = File::create(reg_path)?;
    writeln!(&mut reg_file, "workload;implementation;alpha;C;R2")?;

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
                    scale,
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
    scale: PlotScale,
) -> Result<(), Box<dyn Error>> {
    let mut by_impl: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for rec in recs {
        let t = if use_time_per_op {
            rec.time_per_op
        } else {
            rec.time_total
        };

        if let Some((x, y)) = project_point(rec.n, t, scale) {
            by_impl
                .entry(rec.implementation.clone())
                .or_default()
                .push((x, y));

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }

    if !min_x.is_finite() || !min_y.is_finite() {
        return Ok(());
    }

    // gráfico
    let filename = format!("{}/{}_{}.png", output_dir, workload, scale.tag());
    let root = BitMapBackend::new(&filename, (900, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{workload} — {} + regressão", scale.title()),
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc(scale.x_label())
        .y_desc(scale.y_label(use_time_per_op))
        .label_style(("sans-serif", 15))
        .draw()?;

    let colors = [
        RGBColor(76, 114, 176),
        RGBColor(221, 132, 82),
        RGBColor(85, 168, 104),
        RGBColor(196, 78, 82),
        RGBColor(129, 114, 178),
        RGBColor(147, 120, 96),
        RGBColor(140, 140, 140),
    ];

    for (idx, (impl_name, mut pts)) in by_impl.into_iter().enumerate() {
        if pts.len() < 2 {
            continue;
        }
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();

        let (slope, intercept) = linear_regression(&xs, &ys);
        let r2 = r2_score(&xs, &ys, slope, intercept);
        let color = colors[idx % colors.len()];

        // Exporta (alpha, C) de forma coerente com a escala.
        // - Se Y é log10: C = 10^intercept
        // - Se Y é linear: C = intercept
        let c = match scale {
            // y linear => intercept é o próprio C
            PlotScale::LinLog | PlotScale::LinLin => intercept,
            // y log10 => intercept = log10(C)
            PlotScale::LogLog | PlotScale::LogLin => 10f64.powf(intercept),
        };


        writeln!(reg_out, "{workload};{impl_name};{slope:.4};{c:.4e};{r2:.6}")?;

        // pontos
        chart.draw_series(
            pts.iter()
                .map(|(x, y)| Circle::new((*x, *y), 3, color.filled())),
        )?;

        // reta
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

/// Regressão linear: y = a x + b (centrada na média)
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
