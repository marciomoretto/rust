use text_io::read;

fn main() {
    // ---- Ler inteiros ----
    let n: i32 = read!();
    let m: i32 = read!();

    // ---- Ler floats ----
    let x: f32 = read!();
    let y: f32 = read!();

    // ---- Saída ----
    println!("{} {}", n + m, n - m);
    println!("{:.1} {:.1}", x + y, x - y);
}
