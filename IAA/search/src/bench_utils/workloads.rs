use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Copy)]
pub enum Workload {
    HitMiddle,
    MissLow,
    MissHigh,
}

impl Workload {
    pub fn name(self) -> &'static str {
        match self {
            Workload::HitMiddle => "hit_middle",
            Workload::MissLow => "miss_low",
            Workload::MissHigh => "miss_high",
        }
    }
}

pub const WORKLOADS: &[Workload] = &[
    Workload::HitMiddle,
    Workload::MissLow,
    Workload::MissHigh,
];

/// Gera um vetor ordenado de tamanho n: valores estritamente crescentes.
pub fn make_sorted_vec(n: usize) -> Vec<i32> {
    // Simples e determinístico: 0, 2, 4, 6, ...
    (0..n as i32).map(|i| i * 2).collect()
}

/// Escolhe um x conforme o workload.
pub fn choose_x(a: &[i32], w: Workload, seed: u64) -> i32 {
    let mut rng = StdRng::seed_from_u64(seed);
    match w {
        Workload::HitMiddle => {
            if a.is_empty() {
                0
            } else {
                let idx = a.len() / 2;
                a[idx]
            }
        }
        Workload::MissLow => {
            // algo menor que o mínimo (lembrando que a[0] >= 0)
            rng.gen_range(-10_000..-1)
        }
        Workload::MissHigh => {
            // algo maior que o máximo
            let last = *a.last().unwrap_or(&0);
            last + rng.gen_range(1..10_000)
        }
    }
}
