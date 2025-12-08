use crate::queue::Queue;
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr;

pub struct ArrayQueue<T> {
    ptr: *mut T,
    head: usize,
    len: usize,
    capacity: usize,
}

impl<T> ArrayQueue<T> {
    const DEFAULT_CAPACITY: usize = 4;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let cap = capacity.max(1);
        let layout = Layout::array::<T>(cap).unwrap();

        let ptr = unsafe { alloc(layout) as *mut T };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Self {
            ptr,
            head: 0,
            len: 0,
            capacity: cap,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn grow(&mut self) {
        let old_cap = self.capacity;
        let new_cap = old_cap * 2;

        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = unsafe {
            let p = alloc(new_layout) as *mut T;
            if p.is_null() {
                handle_alloc_error(new_layout);
            }
            p
        };

        // copiar em ordem lógica para 0..len
        unsafe {
            for i in 0..self.len {
                let idx = (self.head + i) % old_cap;
                let src = self.ptr.add(idx);
                let dst = new_ptr.add(i);
                ptr::write(dst, ptr::read(src));
            }

            // liberar o buffer antigo
            let old_layout = Layout::array::<T>(old_cap).unwrap();
            dealloc(self.ptr as *mut u8, old_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_cap;
        self.head = 0;
    }
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn enqueue(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        let cap = self.capacity;
        let tail_idx = (self.head + self.len) % cap;

        unsafe {
            let slot = self.ptr.add(tail_idx);
            ptr::write(slot, value);
        }

        self.len += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let cap = self.capacity;
        let idx = self.head;

        let value = unsafe {
            let front_ptr = self.ptr.add(idx);
            ptr::read(front_ptr)
        };

        self.head = (self.head + 1) % cap;
        self.len -= 1;

        Some(value)
    }

    fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let idx = self.head;
                Some(&*self.ptr.add(idx))
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for ArrayQueue<T> {
    fn drop(&mut self) {
        // dropar exatamente os `len` elementos na ordem lógica
        let cap = self.capacity;
        for i in 0..self.len {
            unsafe {
                let idx = (self.head + i) % cap;
                ptr::drop_in_place(self.ptr.add(idx));
            }
        }

        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
