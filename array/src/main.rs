use text_io::read;

fn marks_summation(marks: &[i32], number_of_students: usize, gender: char) -> i32 {
    // if é uma expressão → devolve 0 ou 1
    let parity = if gender == 'b' { 0 } else { 1 };

    let mut sum = 0;

    for i in 0..number_of_students {
        if i % 2 == parity {
            sum += marks[i];
        }
    }

    sum
}

fn main() {
    let number_of_students: usize = read!();

    // array fixo de tamanho máximo permitido
    let mut marks: [i32; 1000] = [0; 1000];

    for i in 0..number_of_students {
        marks[i] = read!();
    }

    let gender: char = read!();

    println!(
        "{}",
        marks_summation(&marks, number_of_students, gender)
    );
}
