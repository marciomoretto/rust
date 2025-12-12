use plot::linear_regression;

#[test]
fn regression_on_perfect_line() {
    // y = 2x + 1
    let xs = vec![0.0, 1.0, 2.0, 3.0];
    let ys: Vec<f64> = xs.iter().map(|x| 2.0 * x + 1.0).collect();

    let (a, b) = linear_regression(&xs, &ys);
    eprintln!("DEBUG: a = {a}, b = {b}");

    assert!((a - 2.0).abs() < 1e-8);
    assert!((b - 1.0).abs() < 1e-8);
}
