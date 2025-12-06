use text_io::read;

fn max_of_two(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn max_of_four(a: i32, b: i32, c: i32, d: i32) -> i32 {
    max_of_two(max_of_two(a, b), max_of_two(c, d))
}

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();
    let c: i32 = read!();
    let d: i32 = read!();

    println!("{}", max_of_four(a, b, c, d));
}
