use std::io::{self};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut current = String::new();

    for ch in line.chars() {
        if ch.is_whitespace() {
            if !current.is_empty() {
                println!("{}", current);
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    // imprime a última palavra, se houver
    if !current.is_empty() {
        println!("{}", current);
    }
}
