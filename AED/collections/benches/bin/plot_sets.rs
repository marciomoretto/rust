use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use plotters::prelude::*;
use plotters::backend::BitMapBackend;
use plotters::style::RGBColor;

const OUTPUT_DIR: &str = "benches/output";

#[derive(Debug, Deserialize)]
struct Record {
    implementation: String,
    workload: String,
    n: usize,
    time_per_op: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(OUTPUT_DIR)?;

    // Ajusta o nome se o arquivo estiver em outro lugar
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // o seu CSV usa ;
        .from_path(format!("{}/set.csv", OUTPUT_DIR))?;

    let mut records: Vec<Record> = Vec::new();
    for result in rdr.deserialize() {
        let rec: Record = result?;
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

    // Workloads que queremos plotar
    let workloads = [
        "insert_distinct",
        "lookup_hit",
        "lookup_miss",
        "insert_sorted",
    ];

    for &workload in &workloads {
        if let Some(recs) = by_workload.get(workload) {
            if recs.is_empty() {
                continue;
            }
            plot_workload(workload, recs)?;
        }
    }

    Ok(())
}

fn plot_workload(workload: &str, recs: &[Record]) -> Result<(), Box<dyn Error>> {
    // Agrupa por implementação
    let mut by_impl: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    for rec in recs {
        by_impl
            .entry(rec.implementation.clone())
            .or_default()
            .push((rec.n as f64, rec.time_per_op));
    }

    // Descobre intervalos globais de n e tempo
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for points in by_impl.values() {
        for (x, y) in points {
            if *x > 0.0 && *y > 0.0 {
                min_x = min_x.min(*x);
                max_x = max_x.max(*x);
                min_y = min_y.min(*y);
                max_y = max_y.max(*y);
            }
        }
    }

    // Segurança básica para log
    if !min_x.is_finite() || !min_y.is_finite() {
        eprintln!("Workload {workload}: dados insuficientes para plot");
        return Ok(());
    }

    // Gráfico em PNG
    let filename = format!("{}/{}.png", OUTPUT_DIR, workload);
    let root = BitMapBackend::new(&filename, (900, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{workload} — log-log + regressão"),
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            min_x.log10()..max_x.log10(), // eixo x em log10(n)
            min_y.log10()..max_y.log10(), // eixo y em log10(time)
        )?;

    chart
        .configure_mesh()
        .x_desc("log10(n)")
        .y_desc("log10(tempo por operação)")
        .label_style(("sans-serif", 15))
        .draw()?;

    let colors: Vec<RGBColor> = vec![
        RGBColor(76, 114, 176),
        RGBColor(221, 132, 82),
        RGBColor(85, 168, 104),
        RGBColor(196, 78, 82),
        RGBColor(129, 114, 178),
        RGBColor(147, 120, 96),
        RGBColor(140, 140, 140),
    ];




    // Desenhar pontos e regressão para cada implementação
    let mut legend_elems = Vec::new();

    for (idx, (impl_name, mut points)) in by_impl.into_iter().enumerate() {
        // Ordena por n
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Filtra pontos válidos
        let valid: Vec<(f64, f64)> = points
            .into_iter()
            .filter(|(x, y)| *x > 0.0 && *y > 0.0)
            .collect();
        if valid.len() < 2 {
            continue;
        }

        let color = colors[idx % colors.len()];

        // Transforma em log10 para regressão
        let logx: Vec<f64> = valid.iter().map(|(x, _)| x.log10()).collect();
        let logy: Vec<f64> = valid.iter().map(|(_, y)| y.log10()).collect();

        let (slope, intercept) = linear_regression(&logx, &logy);

        // Série de pontos em coordenadas log (já transformadas)
        chart.draw_series(valid.iter().map(|(x, y)| {
            let lx = x.log10();
            let ly = y.log10();
            Circle::new((lx, ly), 3, color.filled())
        }))?;

        // Linha de regressão: usamos os próprios logx como domínio
        let mut reg_points: Vec<(f64, f64)> = logx
            .iter()
            .map(|lx| (*lx, slope * lx + intercept))
            .collect();
        reg_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let coeff = 10f64.powf(intercept);
        let label = format!("{impl_name} (α={:.2}, C≈{:.2e})", slope, coeff);

        // linha de regressão registrada para a legenda
        chart
            .draw_series(std::iter::once(
                PathElement::new(reg_points.clone(), color.stroke_width(1))
            ))?
            .label(label.clone())
            .legend(move |(x, y)| {
                PathElement::new(
                    vec![(x, y), (x + 20, y)],
                    color.stroke_width(2),
                )
            });

        // pontos (opcional incluir como legendáveis — mas só a linha basta)
        chart.draw_series(valid.iter().map(|(x, y)| {
            Circle::new((x.log10(), y.log10()), 4, color.filled())
        }))?;

        legend_elems.push((label, color));
    }

    // Desenhar legenda manualmente
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 15))
        .draw()?;

    // Plotters faz a legenda automática pelos elementos desenhados,
    // mas se quiser algo mais manual, poderia adaptar aqui.

    println!("Gerado: {filename}");
    Ok(())
}

/// Regressão linear simples em (x, y): y ≈ a * x + b
fn linear_regression(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xx: f64 = xs.iter().map(|x| x * x).sum();
    let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();

    let denom = n * sum_xx - sum_x * sum_x;
    if denom == 0.0 {
        return (0.0, 0.0);
    }

    let a = (n * sum_xy - sum_x * sum_y) / denom;
    let b = (sum_y - a * sum_x) / n;
    (a, b)
}
