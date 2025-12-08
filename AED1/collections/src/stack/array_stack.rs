use crate::stack::Stack;
use std::alloc::{alloc, dealloc, realloc, handle_alloc_error, Layout};
use std::ptr;

/// Pilha implementada com arranjo redimensionável manual
pub struct ArrayStack<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> ArrayStack<T> {
    const DEFAULT_CAPACITY: usize = 4;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    /// Cria pilha vazia com capacidade inicial escolhida
    pub fn with_capacity(capacity: usize) -> Self {
        let cap = capacity.max(1);
        let layout = Layout::array::<T>(cap).unwrap();

        let ptr = unsafe { alloc(layout) as *mut T };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Self {
            ptr,
            len: 0,
            capacity: cap,
        }
    }

    /// Número de elementos na pilha
    pub fn len(&self) -> usize {
        self.len
    }

    /// Garante que há capacidade ≥ min_capacity
    fn ensure_capacity(&mut self, min_capacity: usize) {
        if self.capacity >= min_capacity {
            return;
        }

        let mut new_cap = self.capacity.max(1);
        while new_cap < min_capacity {
            new_cap *= 2;
        }

        self.resize_capacity(new_cap);
    }

    /// Se estiver muito vazio, reduz a capacidade
    fn shrink_if_necessary(&mut self) {
        let len = self.len;
        let cap = self.capacity;

        if cap <= Self::DEFAULT_CAPACITY {
            return;
        }

        if len < cap / 4 {
            let mut new_cap = cap / 2;

            if new_cap < Self::DEFAULT_CAPACITY {
                new_cap = Self::DEFAULT_CAPACITY;
            }
            if new_cap < len {
                new_cap = len;
            }

            self.resize_capacity(new_cap);
        }
    }

    /// Redimensiona a capacidade do buffer
    fn resize_capacity(&mut self, new_cap: usize) {
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        unsafe {
            let new_ptr = realloc(
                self.ptr as *mut u8,
                old_layout,
                new_layout.size(),
            ) as *mut T;

            if new_ptr.is_null() {
                handle_alloc_error(new_layout);
            }

            self.ptr = new_ptr;
        }

        self.capacity = new_cap;
    }

    /// Ponteiro mutável para o buffer interno.
    /// (útil para push/pop)
    fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, value: T) {
        let idx = self.len;
        self.ensure_capacity(idx + 1);

        unsafe {
            // escreve em memória que ainda não foi inicializada
            let ptr = self.as_mut_ptr().add(idx);
            ptr::write(ptr, value);
            // agora esse elemento passou a existir logicamente
            self.len = idx + 1;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let idx = self.len - 1;

        let value = unsafe {
            // lê sem rodar Drop duas vezes
            let ptr = self.as_mut_ptr().add(idx);
            ptr::read(ptr)
        };

        self.len = idx;
        self.shrink_if_necessary();

        Some(value)
    }

    fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                Some(&*self.ptr.add(self.len - 1))
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for ArrayStack<T> {
    fn drop(&mut self) {
        // dropar cada elemento inicializado
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i));
            }
        }

        // liberar memória
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
