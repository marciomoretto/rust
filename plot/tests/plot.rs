use plot::plot_from_csv;
use std::fs;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_plotting_pipeline() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("out");
    fs::create_dir_all(&out).unwrap();

    let csv = out.join("dummy.csv");

    let mut f = fs::File::create(&csv).unwrap();
    writeln!(f, "implementation;workload;n;time_per_op").unwrap();
    writeln!(f, "algo;sort_random;10;0.001").unwrap();
    writeln!(f, "algo;sort_random;100;0.01").unwrap();

    let workloads = ["sort_random"];

    // Supondo que plot_from_csv permita passar o output_dir:
    plot_from_csv(csv.to_str().unwrap(), &workloads, out.to_str().unwrap()).unwrap();

    assert!(out.join("sort_random.png").exists());
}

