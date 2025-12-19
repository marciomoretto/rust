use primes::algorithms::primes::is_prime_sqrt;

#[test]
fn handles_small_numbers() {
    assert!(!is_prime_sqrt(0));
    assert!(!is_prime_sqrt(1));
    assert!(is_prime_sqrt(2));
    assert!(is_prime_sqrt(3));
    assert!(!is_prime_sqrt(4));
}

#[test]
fn handles_even_numbers() {
    assert!(!is_prime_sqrt(6));
    assert!(!is_prime_sqrt(100));
    assert!(is_prime_sqrt(2));
}

#[test]
fn known_primes() {
    let primes = [
        5u64, 7, 11, 13, 17, 19, 23, 29,
        97, 101, 9_973, 99_991,
    ];
    for &p in &primes {
        assert!(is_prime_sqrt(p), "should be prime: {p}");
    }
}

#[test]
fn known_composites() {
    let composites = [
        9u64, 15, 21, 25, 27, 33, 35, 49,
        121, 143, 221, 1_001, 9_999, 100_001,
    ];
    for &n in &composites {
        assert!(!is_prime_sqrt(n), "should be composite: {n}");
    }
}

#[test]
fn squares_are_not_prime_except_trivial_cases() {
    for n in 2u64..200 {
        let sq = n * n;
        assert!(!is_prime_sqrt(sq), "square should be composite: {sq}");
    }
}

#[test]
fn product_of_two_primes_is_composite() {
    let ps = [3u64, 5, 7, 11, 13, 17, 19, 23];
    for &a in &ps {
        for &b in &ps {
            let n = a * b;
            assert!(!is_prime_sqrt(n), "product should be composite: {a}*{b}={n}");
        }
    }
}
