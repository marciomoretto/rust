use rand::Rng;

use search::algorithms::{binary::search_binary, linear::search_linear};

fn check(a: &[i32], x: i32, expected: bool) {
    assert_eq!(
        search_linear(a, x),
        expected,
        "linear falhou para a={:?}, x={}",
        a,
        x
    );
    assert_eq!(
        search_binary(a, x),
        expected,
        "binary falhou para a={:?}, x={}",
        a,
        x
    );
}

#[test]
fn test_empty() {
    let a: [i32; 0] = [];
    check(&a, 0, false);
}

#[test]
fn test_singleton() {
    let a = [10];
    check(&a, 10, true);
    check(&a, 9, false);
    check(&a, 11, false);
}

#[test]
fn test_small_hits() {
    let a = [2, 4, 6, 8, 10];
    check(&a, 2, true);   // primeiro
    check(&a, 6, true);   // meio
    check(&a, 10, true);  // último
}

#[test]
fn test_small_misses() {
    let a = [2, 4, 6, 8, 10];
    check(&a, -1, false); // menor que todos
    check(&a, 11, false); // maior que todos
    check(&a, 7, false);  // entre dois elementos
}

#[test]
fn test_random_sorted_agreement() {
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let n: usize = rng.gen_range(0..500);

        // vetor ordenado e estritamente crescente (0, 2, 4, ...)
        let a: Vec<i32> = (0..n as i32).map(|i| i * 2).collect();

        // escolhe um x qualquer num intervalo que gera hit e miss
        let x: i32 = rng.gen_range(-1000..(2 * n as i32 + 1000));

        let r_linear = search_linear(&a, x);
        let r_binary = search_binary(&a, x);

        assert_eq!(
            r_linear, r_binary,
            "Resultados diferentes para a={:?}, x={}",
            a, x
        );

        // opcional: ancora na implementação padrão da std
        let std_ans = a.binary_search(&x).is_ok();
        assert_eq!(r_binary, std_ans, "binary divergiu da std");
    }
}
