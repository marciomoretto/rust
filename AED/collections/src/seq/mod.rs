use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum SeqError {
    OutOfBounds { index: usize, len: usize },
    Inconsistent,
}

pub trait Seq<T>: Index<usize, Output = T> + IndexMut<usize, Output = T> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insere em uma posição (0 <= index <= len).
    fn insert_at(&mut self, index: usize, value: T) -> Result<(), SeqError>;

    /// Remove da posição dada.
    fn remove_from(&mut self, index: usize) -> Option<T>;

    /// Implementação padrão de append: insere no final usando insert_at.
    fn append(&mut self, value: T) {
        self
            .insert_at(self.len(), value)
            .expect("append: insert_at falhou com índice len()");
    }

    /// Implementação padrão de prepend: insere no começo usando insert_at.
    fn prepend(&mut self, value: T) {
        self
            .insert_at(0, value)
            .expect("prepend: insert_at falhou com índice 0");
    }

    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut T> + 'a>;
}

pub mod array_seq;
pub mod list_seq;

pub use array_seq::ArraySeq;
pub use list_seq::ListSeq;
