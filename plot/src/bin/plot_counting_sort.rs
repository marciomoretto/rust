use plotters::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};

const OUTPUT_DIR: &str = "../IAA/sorting/benches/output";
const SCALE: f64 = 1e6; // <- come 6 zeros em n e k no gráfico

#[derive(Debug, Clone)]
struct Row {
    implementation: String,
    n: f64,
    k: f64,
    time_total: f64,
}

fn main() {
    let csv_path = format!("{OUTPUT_DIR}/linear.csv");

    if let Err(e) = run(&csv_path) {
        eprintln!("erro ao gerar regressão/plot 3D: {e}");
        std::process::exit(1);
    }
}

fn run(csv_path: &str) -> Result<(), Box<dyn Error>> {
    let (list, selected) = parse_args();

    if list {
        for a in list_implementations(csv_path)? {
            println!("{a}");
        }
        return Ok(());
    }

    let mut rows = read_csv(csv_path)?;
    if !selected.is_empty() {
        rows.retain(|r| selected.contains(&r.implementation));
    }

    fs::create_dir_all(OUTPUT_DIR)?;

    // quais algos existem no dataset filtrado?
    let mut algos = BTreeSet::new();
    for r in &rows {
        algos.insert(r.implementation.clone());
    }

    // CSV único com uma regressão (plano) por algoritmo
    let reg_path = format!("{OUTPUT_DIR}/linear_regression.csv");
    let reg_file = fs::File::create(&reg_path)?;
    let mut reg_out = BufWriter::new(reg_file);

    writeln!(reg_out, "implementation;mode;points;a;b;c;r2")?;

    for algo in algos {
        let data: Vec<Row> = rows.iter().filter(|r| r.implementation == algo).cloned().collect();
        if data.len() < 3 {
            eprintln!("aviso: poucos pontos para {algo} ({}). Pulando.", data.len());
            continue;
        }

        let (a, b, c, r2) = fit_plane_ols(&data)?;
        writeln!(
            reg_out,
            "{};2d_plane;{};{:.10e};{:.10e};{:.10e};{:.6}",
            algo,
            data.len(),
            a,
            b,
            c,
            r2
        )?;

        plot_3d_plane(OUTPUT_DIR, &algo, &data, a, b, c)?;
    }

    reg_out.flush()?;
    eprintln!("regressões salvas em: {reg_path}");
    Ok(())
}

//
// ---------- plot 3D (pontos + wireframe do plano) ----------
//

fn plot_3d_plane(
    out_dir: &str,
    algo: &str,
    rows: &[Row],
    a: f64,
    b: f64,
    c: f64,
) -> Result<(), Box<dyn Error>> {
    // ranges em unidades ORIGINAIS (n,k), porque lerp e o plano trabalham com n,k reais
    let (n_min, n_max, k_min, k_max, t_min, t_max) = ranges(rows);
    let (t_min, t_max) = pad_z(t_min, t_max);

    let filename = format!("{out_dir}/{algo}.plane3d.png");
    let root = BitMapBackend::new(&filename, (1400, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    // eixo do gráfico em unidades ESCALADAS (milhões)
    let mut chart = ChartBuilder::on(&root)
        .caption(algo, ("sans-serif", 26))
        .margin(15)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_3d(
            (n_min / SCALE)..(n_max / SCALE),
            (k_min / SCALE)..(k_max / SCALE),
            t_min..t_max,
        )?;

    chart.configure_axes().draw()?;

    // rótulos manuais (2D)
    root.draw(&Text::new(
        "n (×10⁶)",
        (85, 865),
        ("sans-serif", 18).into_font(),
    ))?;
    root.draw(&Text::new(
        "k (×10⁶)",
        (35, 835),
        ("sans-serif", 18).into_font(),
    ))?;
    root.draw(&Text::new(
        "tempo (s)",
        (25, 70),
        ("sans-serif", 18).into_font(),
    ))?;

    let base_color = Palette99::pick(0);

    // pontos (ESCALADOS em n,k)
    let pts_style = base_color.filled();
    chart.draw_series(rows.iter().map(|r| {
        Circle::new(
            (r.n / SCALE, r.k / SCALE, r.time_total),
            3,
            pts_style.clone(),
        )
    }))?;

    // wireframe do plano (mesma cor, mais “leve”)
    let plane_width: u32 = 1;
    let plane_style = base_color.mix(0.3).stroke_width(plane_width);

    let grid_density = 64usize;
    let n_slices = grid_density;
    let k_slices = grid_density;

    // linhas paralelas ao eixo n, para vários k fixos
    for i in 0..=k_slices {
        let kk = lerp(k_min, k_max, i as f64 / k_slices as f64);

        // plano calculado em n,k ORIGINAIS
        let z0 = a * n_min + b * kk + c;
        let z1 = a * n_max + b * kk + c;

        // mas coordenadas enviadas ao gráfico são ESCALADAS
        let p0 = (n_min / SCALE, kk / SCALE, z0);
        let p1 = (n_max / SCALE, kk / SCALE, z1);

        chart.draw_series(std::iter::once(PathElement::new(
            vec![p0, p1],
            plane_style.clone(),
        )))?;
    }

    // linhas paralelas ao eixo k, para vários n fixos
    for i in 0..=n_slices {
        let nn = lerp(n_min, n_max, i as f64 / n_slices as f64);

        let z0 = a * nn + b * k_min + c;
        let z1 = a * nn + b * k_max + c;

        let p0 = (nn / SCALE, k_min / SCALE, z0);
        let p1 = (nn / SCALE, k_max / SCALE, z1);

        chart.draw_series(std::iter::once(PathElement::new(
            vec![p0, p1],
            plane_style.clone(),
        )))?;
    }

    eprintln!("salvo: {filename}");
    Ok(())
}

fn ranges(rows: &[Row]) -> (f64, f64, f64, f64, f64, f64) {
    let mut n_min = f64::INFINITY;
    let mut n_max = f64::NEG_INFINITY;
    let mut k_min = f64::INFINITY;
    let mut k_max = f64::NEG_INFINITY;
    let mut t_min = f64::INFINITY;
    let mut t_max = f64::NEG_INFINITY;

    for r in rows {
        n_min = n_min.min(r.n);
        n_max = n_max.max(r.n);
        k_min = k_min.min(r.k);
        k_max = k_max.max(r.k);
        t_min = t_min.min(r.time_total);
        t_max = t_max.max(r.time_total);
    }
    (n_min, n_max, k_min, k_max, t_min, t_max)
}

fn pad_z(lo: f64, hi: f64) -> (f64, f64) {
    let span = (hi - lo).max(1e-12);
    let p = 0.05 * span;
    (lo - p, hi + p)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

//
// ---------- regressão 2D: T = a n + b k + c ----------
//

fn fit_plane_ols(rows: &[Row]) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let mut s_nn = 0.0;
    let mut s_nk = 0.0;
    let mut s_n1 = 0.0;
    let mut s_kk = 0.0;
    let mut s_k1 = 0.0;
    let mut s_11 = 0.0;

    let mut s_nt = 0.0;
    let mut s_kt = 0.0;
    let mut s_1t = 0.0;

    for r in rows {
        let n = r.n;
        let k = r.k;
        let t = r.time_total;

        s_nn += n * n;
        s_nk += n * k;
        s_n1 += n;

        s_kk += k * k;
        s_k1 += k;

        s_11 += 1.0;

        s_nt += n * t;
        s_kt += k * t;
        s_1t += t;
    }

    let (a, b, c) = solve_3x3(
        [[s_nn, s_nk, s_n1], [s_nk, s_kk, s_k1], [s_n1, s_k1, s_11]],
        [s_nt, s_kt, s_1t],
    )?;

    let m = rows.len() as f64;
    let mean_t = s_1t / m;

    let mut ss_tot = 0.0;
    let mut ss_res = 0.0;
    for r in rows {
        let t_hat = a * r.n + b * r.k + c;
        let dt = r.time_total - mean_t;
        ss_tot += dt * dt;

        let err = r.time_total - t_hat;
        ss_res += err * err;
    }
    let r2 = if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 1.0 };

    Ok((a, b, c, r2))
}

fn solve_3x3(a: [[f64; 3]; 3], b: [f64; 3]) -> Result<(f64, f64, f64), Box<dyn Error>> {
    let mut m = [
        [a[0][0], a[0][1], a[0][2], b[0]],
        [a[1][0], a[1][1], a[1][2], b[1]],
        [a[2][0], a[2][1], a[2][2], b[2]],
    ];

    for col in 0..3 {
        let mut piv = col;
        for r in (col + 1)..3 {
            if m[r][col].abs() > m[piv][col].abs() {
                piv = r;
            }
        }
        if m[piv][col].abs() < 1e-30 {
            return Err("matriz singular (pivot ~ 0)".into());
        }
        if piv != col {
            m.swap(piv, col);
        }

        let div = m[col][col];
        for j in col..4 {
            m[col][j] /= div;
        }

        for r in 0..3 {
            if r == col {
                continue;
            }
            let factor = m[r][col];
            for j in col..4 {
                m[r][j] -= factor * m[col][j];
            }
        }
    }

    Ok((m[0][3], m[1][3], m[2][3]))
}

//
// ---------- CLI + CSV ----------
//

fn parse_args() -> (bool, HashSet<String>) {
    let mut list = false;
    let mut selected: HashSet<String> = HashSet::new();

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
            _ => {}
        }
    }
    (list, selected)
}

fn list_implementations(csv_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(csv_path)?;

    let headers = rdr.headers()?.clone();
    let i_impl = headers
        .iter()
        .position(|h| h.trim() == "implementation")
        .ok_or("CSV sem coluna 'implementation'")?;

    let mut set = BTreeSet::new();
    for rec in rdr.records() {
        let rec = rec?;
        if let Some(name) = rec.get(i_impl) {
            let name = name.trim();
            if !name.is_empty() {
                set.insert(name.to_string());
            }
        }
    }
    Ok(set.into_iter().collect())
}

fn read_csv(csv_path: &str) -> Result<Vec<Row>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(csv_path)?;

    let headers = rdr.headers()?.clone();
    let pos = |name: &str| headers.iter().position(|h| h.trim() == name);

    let i_impl = pos("implementation").ok_or("CSV sem coluna 'implementation'")?;
    let i_n = pos("n").ok_or("CSV sem coluna 'n'")?;
    let i_k = pos("k").ok_or("CSV sem coluna 'k'")?;
    let i_tt = pos("time_total").ok_or("CSV sem coluna 'time_total'")?;

    let mut out = Vec::new();
    for rec in rdr.records() {
        let rec = rec?;

        let implementation = rec.get(i_impl).unwrap_or("").trim().to_string();
        let n: f64 = rec.get(i_n).unwrap_or("0").trim().parse().unwrap_or(0.0);
        let k: f64 = rec.get(i_k).unwrap_or("0").trim().parse().unwrap_or(0.0);
        let time_total: f64 = rec.get(i_tt).unwrap_or("0").trim().parse().unwrap_or(0.0);

        if !implementation.is_empty()
            && n > 0.0
            && k > 0.0
            && time_total.is_finite()
            && time_total > 0.0
        {
            out.push(Row {
                implementation,
                n,
                k,
                time_total,
            });
        }
    }
    Ok(out)
}
