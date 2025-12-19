#[derive(Clone, Copy)]
pub struct Workload {
    pub name: &'static str,
    pub make_number: fn(u64) -> u64,
}

// “quase quadrado”: n^2 + 1
fn near_square_prime(n: u64) -> u64 {
    n.saturating_mul(n).saturating_add(1)
}

// composto com fator perto da raiz: n*(n+2)
fn near_square_composite(n: u64) -> u64 {
    n.saturating_mul(n.saturating_add(2))
}

pub const WORKLOADS: &[Workload] = &[
    Workload { name: "near_square_prime", make_number: near_square_prime },
    Workload { name: "near_square_composite", make_number: near_square_composite },
];
