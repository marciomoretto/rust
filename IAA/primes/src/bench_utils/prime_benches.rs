use crate::algorithms::PrimeAlgo;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

/// Gera um primo ímpar com exatamente `bits` bits (isto é, em [2^(bits-1), 2^bits)).
fn gen_prime_with_bits<A: PrimeAlgo>(bits: u32, rng: &mut StdRng) -> u64 {
    assert!(bits >= 2 && bits <= 63);

    let lo = 1u64 << (bits - 1);
    let hi = 1u64 << bits;

    loop {
        let mut x = rng.gen_range(lo..hi);
        x |= 1;   // ímpar
        x |= lo;  // garante 'bits' bits

        if A::is_prime(x) {
            return x;
        }
    }
}

/// Bench “pior caso”: n primo => o laço vai até sqrt(n).
/// Imprime uma linha: implementation;workload;n;time_total;time_per_op
///
/// Aqui usamos `n = bits` para o CSV ficar “bits no eixo x”.
pub fn bench_prime_worst_case<A: PrimeAlgo>(bits: u32, reps: usize) {
    let mut rng = StdRng::seed_from_u64(0xC0FFEE_u64 ^ (bits as u64));
    let p = gen_prime_with_bits::<A>(bits, &mut rng);

    let start = Instant::now();
    let mut sink: u64 = 0;

    for _ in 0..reps {
        // blinda o compilador: impede mover/combinar chamadas
        let pp = std::hint::black_box(p);

        // blinda o resultado também
        let r = std::hint::black_box(A::is_prime(pp));

        sink = sink.wrapping_add(r as u64);
    }

    std::hint::black_box(sink);

    let time_total = start.elapsed().as_secs_f64();
    let time_per_op = time_total / (reps as f64);

    println!(
        "{};{};{};{};{}",
        A::name(),
        "prime_worst_case_bits",
        p,
        time_total,
        time_per_op
    );
}
