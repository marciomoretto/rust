use text_io::read;

fn main() {
    let mut n: i32 = read!();
    let mut soma = 0;

    while n > 0 {
        soma += n % 10; // pega o último dígito
        n /= 10;        // descarta o último dígito
    }

    println!("{}", soma);
}

