pub trait Set<T> {
    /// Insere o elemento. Retorna `true` se de fato inseriu (não existia),
    /// `false` se o elemento já estava no conjunto.
    fn insert(&mut self, value: T) -> bool;

    /// Remove o elemento. Retorna `true` se removeu, `false` se não estava presente.
    fn remove(&mut self, value: &T) -> bool;

    /// Verifica se o elemento está no conjunto.
    fn contains(&self, value: &T) -> bool;

    /// Número de elementos no conjunto.
    fn len(&self) -> usize;

    /// Conjunto vazio?
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SetName {
    fn name() -> &'static str;
}

pub mod list_set;

pub mod bst_set;
pub mod bst_avl_set;
pub mod bst_rb_set;

pub mod hash_chaining_set;
pub mod hash_probing_set;

pub use list_set::ListSet;
pub use bst_set::BstSet;
pub use bst_avl_set::BstAvlSet;
pub use bst_rb_set::BstRBSet;

pub use hash_chaining_set::HashChainingSet;
pub use hash_probing_set::HashProbingSet;
