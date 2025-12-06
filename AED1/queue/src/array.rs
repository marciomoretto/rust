use crate::tad::{Queue, QueueError};

#[derive(Debug)]
pub struct ArrayQueue<T, const N: usize> {
    data: [Option<T>; N],
    head: usize, // índice do primeiro elemento
    tail: usize, // índice da próxima posição livre
    len: usize,  // número de elementos na fila
}

impl<T, const N: usize> ArrayQueue<T, N> {
    pub fn new() -> Self {
        Self {
            data: [(); N].map(|_| None),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }
}

impl<T, const N: usize> Queue<T> for ArrayQueue<T, N> {
    fn enqueue(&mut self, value: T) -> Result<(), QueueError> {
        if self.is_full() {
            Err(QueueError::Overflow)
        } else {
            self.data[self.tail] = Some(value);
            self.tail = (self.tail + 1) % N;
            self.len += 1;
            Ok(())
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let old_head = self.head;
            self.head = (self.head + 1) % N;
            self.len -= 1;
            self.data[old_head].take()
        }
    }

    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.head].as_ref()
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}
