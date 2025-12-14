use std::time::{Duration, Instant};

/// Mede ns/op chamando `f()` `reps` vezes.
pub fn time_it_ns_per_op(mut reps: u64, mut f: impl FnMut()) -> f64 {
    // Aumenta reps até passar um limiar de tempo, para reduzir ruído.
    let min_total = Duration::from_millis(200);

    loop {
        let t0 = Instant::now();
        for _ in 0..reps {
            f();
        }
        let dt = t0.elapsed();
        if dt >= min_total || reps >= 1_000_000_000 {
            return (dt.as_nanos() as f64) / (reps as f64);
        }
        reps *= 2;
    }
}
