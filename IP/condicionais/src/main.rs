use text_io::read;

fn main() {
    let n: i32 = read!();

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
        _ => println!("Greater than 9"),
    }
}

