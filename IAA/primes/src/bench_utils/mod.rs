pub mod prime_benches;

// reexporta no “glob import”
pub use prime_benches::bench_prime_worst_case;
pub mod core;
pub mod workloads;
