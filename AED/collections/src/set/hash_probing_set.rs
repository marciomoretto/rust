use crate::set::Set;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

/// Conjunto implementado com tabela de hash usando encadeamento.
///
/// - `buckets` é um vetor de listas.
/// - Cada lista contém os elementos que caíram naquele bucket.
/// - Não permite elementos repetidos.
/// - `S` é o tipo responsável por construir hashers (ex.: `RandomState`, `FxBuildHasher`, etc.).
#[derive(Debug)]
pub struct HashProbingSet<T, S = RandomState> {
    buckets: Vec<Vec<T>>,
    len: usize,
    hash_builder: S,
}

// Construtor padrão: usa o mesmo hasher do HashMap (`RandomState`)
impl<T> HashProbingSet<T, RandomState> {
    pub fn new() -> Self {
        Self::with_hasher(RandomState::new())
    }
}

// Implementação genérica em qualquer `S: BuildHasher`
impl<T, S> HashProbingSet<T, S>
where
    S: BuildHasher,
{
    const INITIAL_BUCKETS: usize = 16;
    const MAX_LOAD_FACTOR: f64 = 0.75;

    /// Cria um HashSet com um construtor de hasher customizado.
    ///
    /// Exemplo:
    /// ```ignore
    /// use rustc_hash::FxBuildHasher;
    /// let mut s: HashSet<i32, FxBuildHasher> =
    ///     HashSet::with_hasher(FxBuildHasher::default());
    /// ```
    pub fn with_hasher(hash_builder: S) -> Self {
        let buckets = (0..Self::INITIAL_BUCKETS)
            .map(|_| Vec::new())
            .collect();

        Self {
            buckets,
            len: 0,
            hash_builder,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn bucket_index(&self, value: &T) -> usize
    where
        T: Hash,
    {
        let mut hasher = self.hash_builder.build_hasher();
        value.hash(&mut hasher);
        let h = hasher.finish() as usize;
        h % self.buckets.len()
    }

    fn load_factor(&self) -> f64 {
        if self.buckets.is_empty() {
            0.0
        } else {
            self.len as f64 / self.buckets.len() as f64
        }
    }

    fn maybe_resize(&mut self)
    where
        T: Eq + Hash,
    {
        if self.load_factor() <= Self::MAX_LOAD_FACTOR {
            return;
        }

        let new_bucket_count = self.buckets.len() * 2;
        let mut new_buckets: Vec<Vec<T>> =
            (0..new_bucket_count).map(|_| Vec::new()).collect();

        for bucket in self.buckets.iter_mut() {
            for value in bucket.drain(..) {
                let mut hasher = self.hash_builder.build_hasher();
                value.hash(&mut hasher);
                let h = hasher.finish() as usize;
                let idx = h % new_bucket_count;
                new_buckets[idx].push(value);
            }
        }

        self.buckets = new_buckets;
        // self.len continua o mesmo
    }
}

// ========================
// impl Set<T> for HashSet<T, S>
// ========================

impl<T, S> Set<T> for HashProbingSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn insert(&mut self, value: T) -> bool {
        let idx = self.bucket_index(&value);
        let bucket = &mut self.buckets[idx];

        if bucket.iter().any(|x| x == &value) {
            return false;
        }

        bucket.push(value);
        self.len += 1;

        self.maybe_resize();

        true
    }

    fn remove(&mut self, value: &T) -> bool {
        let idx = self.bucket_index(value);
        let bucket = &mut self.buckets[idx];

        if let Some(pos) = bucket.iter().position(|x| x == value) {
            bucket.swap_remove(pos);
            self.len -= 1;
            true
        } else {
            false
        }
    }

    fn contains(&self, value: &T) -> bool {
        let idx = self.bucket_index(value);
        self.buckets[idx].iter().any(|x| x == value)
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<T> crate::set::SetName for HashProbingSet<T> {
    fn name() -> &'static str {
        "HashProbingSet"
    }
}

impl<T> Default for HashProbingSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
