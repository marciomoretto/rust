use crate::stack::Stack;

/// Nó da lista ligada simples
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

/// Pilha implementada como lista ligada (topo = cabeça da lista)
pub struct ListStack<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> ListStack<T> {
    /// Cria uma pilha vazia
    pub fn new() -> Self {
        ListStack {
            head: None,
            len: 0,
        }
    }

    /// Número de elementos na pilha
    pub fn len(&self) -> usize {
        self.len
    }

    /// Conveniência, espelho do trait
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Stack<T> for ListStack<T> {
    fn push(&mut self, value: T) {
        // novo nó aponta para o antigo head
        let new_node = Box::new(Node {
            elem: value,
            next: self.head.take(),
        });

        // e vira o novo head
        self.head = Some(new_node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // take "arranca" o head e bota None no lugar
        self.head.take().map(|boxed_node| {
            let Node { elem, next } = *boxed_node;
            self.head = next;
            self.len -= 1;
            elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}
