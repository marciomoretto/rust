use text_io::read;

fn s(n: i32, a: i32, b: i32, c: i32) -> i32 {
    match n {
        1 => a,
        2 => b,
        3 => c,
        _ => s(n - 1, a, b, c) + s(n - 2, a, b, c) + s(n - 3, a, b, c),
    }
}

fn main() {
    let n: i32 = read!();
    let a: i32 = read!();
    let b: i32 = read!();
    let c: i32 = read!();

    println!("{}", s(n, a, b, c));
}
