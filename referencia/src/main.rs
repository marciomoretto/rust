use text_io::read;

fn update(a: &mut i32, b: &mut i32) {
    let a_old = *a;
    let b_old = *b;

    *a = a_old + b_old;
    *b = (a_old - b_old).abs();
}

fn main() {
    let mut a: i32 = read!();
    let mut b: i32 = read!();

    update(&mut a, &mut b);

    println!("{}", a);
    println!("{}", b);
}
