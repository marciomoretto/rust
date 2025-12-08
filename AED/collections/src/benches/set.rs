use crate::set::{Set, SetName};
use super::core::time_it;
use rand::prelude::*;

pub fn bench_insert_distinct<S>(n: usize)
where
    S: Set<i32> + SetName + Default,
{
    // n inserções distintas
    time_it(S::name(), "insert_distinct", n, n, || {
        let mut set = S::default();

        // valores embaralhados
        let mut values: Vec<i32> = (0..n as i32).collect();
        values.shuffle(&mut thread_rng());

        for v in values {
            set.insert(v);
        }
    });
}

pub fn bench_insert_sorted<S>(n: usize)
where
    S: Set<i32> + SetName + Default,
{
    // n inserções em ordem crescente
    time_it(S::name(), "insert_sorted", n, n, || {
        let mut set = S::default();

        for v in 0..n as i32 {
            set.insert(v);
        }
    });
}

pub fn bench_lookup_hit<S>(n: usize)
where
    S: Set<i32> + SetName + Default,
{
    // n lookups bem-sucedidos
    time_it(S::name(), "lookup_hit", n, n, || {
        let mut set = S::default();

        let mut values: Vec<i32> = (0..n as i32).collect();
        values.shuffle(&mut thread_rng());

        // fase de construção (não entra em ops, mas entra no tempo total)
        for v in &values {
            set.insert(*v);
        }

        // buscas embaralhadas (essas são as n operações que estamos contando)
        values.shuffle(&mut thread_rng());
        for v in values {
            set.contains(&v);
        }
    });
}

pub fn bench_lookup_miss<S>(n: usize)
where
    S: Set<i32> + SetName + Default,
{
    // n lookups malsucedidos
    time_it(S::name(), "lookup_miss", n, n, || {
        let mut set = S::default();

        let mut values: Vec<i32> = (0..n as i32).collect();
        values.shuffle(&mut thread_rng());

        // fase de construção
        for v in &values {
            set.insert(*v);
        }

        // misses também randomizados
        let mut misses: Vec<i32> = ((n as i32)..(2 * n as i32)).collect();
        misses.shuffle(&mut thread_rng());

        // essas n buscas são as ops
        for v in misses {
            set.contains(&v);
        }
    });
}
