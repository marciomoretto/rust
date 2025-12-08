use std::ops::{Index, IndexMut};
use std::marker::PhantomData;

use super::{Seq, SeqError};

// ========================
// Nó da lista ligada
// ========================

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// ========================
// Estrutura principal: ListSeq<T>
// ========================

pub struct ListSeq<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> ListSeq<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }
}

// ========================
// Iterador imutável
// ========================

pub struct ListSeqIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListSeqIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.data)
    }
}

// ========================
// Iterador mutável
// ========================

pub struct ListSeqIterMut<'a, T> {
    next: *mut Node<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> ListSeqIterMut<'a, T> {
    fn new(start: Option<&'a mut Node<T>>) -> Self {
        let ptr = match start {
            Some(node) => node as *mut Node<T>,
            None => std::ptr::null_mut(),
        };

        Self {
            next: ptr,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ListSeqIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }

        unsafe {
            let node = &mut *self.next;

            self.next = match node.next.as_deref_mut() {
                Some(next) => next as *mut Node<T>,
                None => std::ptr::null_mut(),
            };

            Some(&mut node.data)
        }
    }
}

// ========================
// Implementação do TAD Seq<T>
// ========================

impl<T> Seq<T> for ListSeq<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn insert_at(&mut self, index: usize, value: T) -> Result<(), SeqError> {
        if index > self.len {
            return Err(SeqError::OutOfBounds { index, len: self.len });
        }

        if index == 0 {
            let new_node = Box::new(Node {
                data: value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            self.len += 1;
            return Ok(());
        }

        let mut i = 0;
        let mut current = self.head.as_mut().ok_or(SeqError::Inconsistent)?;

        while i + 1 < index {
            current = current.next.as_mut().ok_or(SeqError::Inconsistent)?;
            i += 1;
        }

        let new_node = Box::new(Node {
            data: value,
            next: current.next.take(),
        });

        current.next = Some(new_node);
        self.len += 1;

        Ok(())
    }

    fn remove_from(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        if index == 0 {
            let head = self.head.take()?;
            self.head = head.next;
            self.len -= 1;
            return Some(head.data);
        }

        let mut i = 0;
        let mut current = self.head.as_mut().unwrap();

        while i + 1 < index {
            current = current.next.as_mut().unwrap();
            i += 1;
        }

        let removed = current.next.take().unwrap();
        current.next = removed.next;
        self.len -= 1;

        Some(removed.data)
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(ListSeqIter {
            next: self.head.as_deref(),
        })
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut T> + 'a> {
        Box::new(ListSeqIterMut::new(self.head.as_deref_mut()))
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        self.iter().nth(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        let mut i = 0;
        let mut current = self.head.as_deref_mut();

        while let Some(node) = current {
            if i == index {
                return Some(&mut node.data);
            }
            current = node.next.as_deref_mut();
            i += 1;
        }

        None
    }
}

// ========================
// Index e IndexMut
// ========================

impl<T> Index<usize> for ListSeq<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect("ListSeq::index: índice fora dos limites")
    }
}

impl<T> IndexMut<usize> for ListSeq<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("ListSeq::index_mut: índice fora dos limites")
    }
}
