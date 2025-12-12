use sorting::algorithms::*;

/// Função auxiliar: aplica o algoritmo S e verifica se ordena igual à std.
fn check_sort<S: Sorter<i32>>(input: &[i32]) {
    let mut v = input.to_vec();
    S::sort(&mut v);

    let mut expected = input.to_vec();
    expected.sort();

    assert_eq!(
        v, expected,
        "algoritmo {} falhou para entrada {:?}",
        S::name(),
        input
    );
}

/// Roda a mesma entrada para todos os algoritmos
fn check_all(input: &[i32]) {
    check_sort::<Bubble>(input);
    check_sort::<Insertion>(input);
    check_sort::<Selection>(input);
    check_sort::<MergeSort>(input);
    check_sort::<QuickSort>(input);
    check_sort::<HeapSort>(input); 
    check_sort::<CountingSort>(input);
    check_sort::<RadixSort>(input);
    check_sort::<RustStd>(input);
}

#[test]
fn sort_empty() {
    check_all(&[]);
}

#[test]
fn sort_single_element() {
    check_all(&[42]);
}

#[test]
fn sort_already_sorted() {
    check_all(&[1, 2, 3, 4, 5]);
}

#[test]
fn sort_reversed() {
    check_all(&[5, 4, 3, 2, 1]);
}

#[test]
fn sort_with_duplicates() {
    check_all(&[3, 1, 2, 3, 2, 1, 1, 3]);
}

#[test]
fn sort_with_negatives() {
    check_all(&[0, -1, 5, -10, 3, 3, -1]);
}

#[test]
fn sort_longer_example() {
    check_all(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn sort_random_arrays() {
    use rand::prelude::*;
    let mut rng = StdRng::seed_from_u64(123);

    for _ in 0..200 {
        let len = rng.gen_range(0..50);
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(rng.gen_range(-100..=100));
        }
        check_all(&v);
    }
}
