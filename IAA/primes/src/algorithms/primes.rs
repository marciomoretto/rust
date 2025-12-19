pub trait PrimeAlgo {
    fn name() -> &'static str;
    fn is_prime(n: u64) -> bool;
}

/// Trial division até sqrt(n), pulando pares.
pub struct TrialSqrt;

impl PrimeAlgo for TrialSqrt {
    fn name() -> &'static str { "TrialSqrt" }

    #[inline(never)]
    fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n % 2 == 0 { return n == 2; }
        let mut d = 3u64;
        while d * d <= n {
            if n % d == 0 { return false; }
            d += 2;
        }
        true
    }
}
