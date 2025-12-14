use duplicates::algorithms::{DuplicateChecker, Naive, LinearSeen};
use rand::Rng;

fn check(slice: &[usize], expected: bool) {
    assert_eq!(
        Naive::has_duplicate(slice),
        expected,
        "Naive falhou em {:?}",
        slice
    );
    assert_eq!(
        LinearSeen::has_duplicate(slice),
        expected,
        "LinearSeen falhou em {:?}",
        slice
    );
}

#[test]
fn sem_repeticao_casos_basicos() {
    check(&[], false);
    check(&[0], false);
    check(&[0, 1], false);
    check(&[0, 1, 2, 3], false);
}

#[test]
fn com_repeticao_casos_basicos() {
    check(&[0, 0], true);
    check(&[0, 1, 0], true);
    check(&[1, 2, 3, 1], true);
    check(&[2, 2, 2], true);
}

#[test]
fn pior_caso_sem_repeticao() {
    let n = 20;
    let v: Vec<usize> = (0..n).collect();
    check(&v, false);
}

#[test]
fn random_os_dois_algoritmos_concordam() {
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let n = rng.gen_range(1..=80);

        // valores garantidos em 0..n-1 (pré-condição do LinearSeen)
        let v: Vec<usize> =
            (0..n).map(|_| rng.gen_range(0..n)).collect();

        let r_naive = Naive::has_duplicate(&v);
        let r_linear = LinearSeen::has_duplicate(&v);

        assert_eq!(
            r_naive, r_linear,
            "Resultados diferentes para v = {:?}",
            v
        );
    }
}
