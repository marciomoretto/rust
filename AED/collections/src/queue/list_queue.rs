use crate::queue::Queue;
use std::ptr;

/// Nó da lista ligada
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

/// Fila implementada como lista ligada (head = frente, tail = fim)
pub struct ListQueue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>, // ponteiro cru para o último nó
    len: usize,
}

impl<T> ListQueue<T> {
    /// Cria uma fila vazia
    pub fn new() -> Self {
        ListQueue {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    /// Número de elementos
    pub fn len(&self) -> usize {
        self.len
    }

}

impl<T> Queue<T> for ListQueue<T> {
    fn enqueue(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            elem: value,
            next: None,
        });

        // ponteiro cru para o Node dentro do Box
        let new_node_raw: *mut Node<T> = &mut *new_node;

        if self.head.is_none() {
            // fila vazia: head e tail apontam para o mesmo nó
            self.head = Some(new_node);
            self.tail = new_node_raw;
        } else {
            unsafe {
                // self.tail sempre aponta para um Node válido nesse ramo
                (*self.tail).next = Some(new_node);
            }
            self.tail = new_node_raw;
        }

        self.len += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        // tira o head
        self.head.take().map(|boxed_node| {
            let Node { elem, next } = *boxed_node;

            // atualiza o head
            self.head = next;

            // se ficou vazia, zera tail
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            self.len -= 1;
            elem
        })
    }

    fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

}
