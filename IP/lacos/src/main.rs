use text_io::read;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();

    for n in a..=b {
        match n {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            5 => println!("five"),
            6 => println!("six"),
            7 => println!("seven"),
            8 => println!("eight"),
            9 => println!("nine"),
            _ => {
                if n % 2 == 0 {
                    println!("even");
                } else {
                    println!("odd");
                }
            }
        }
    }
}
