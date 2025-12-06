use crate::tad::{Queue, QueueError};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct ListQueue<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> ListQueue<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
}

impl<T> Queue<T> for ListQueue<T> {
    fn enqueue(&mut self, value: T) -> Result<(), QueueError> {
        let new_node = Box::new(Node { value, next: None });

        match self.head.as_mut() {
            // Caso 1: fila vazia — head vira o novo nó
            None => {
                self.head = Some(new_node);
            }
            // Caso 2: fila não vazia — percorre até o final
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let Node { value, next } = *boxed_node;
            self.head = next;
            value
        })
    }

    fn front(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
