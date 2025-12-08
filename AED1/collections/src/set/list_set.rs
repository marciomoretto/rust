use crate::set::Set;

// Nó da lista ligada
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

/// Conjunto implementado como lista ligada.
/// Não permite elementos repetidos.
/// Operações são O(n).
pub struct ListSet<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> ListSet<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: PartialEq> Set<T> for ListSet<T> {
    fn insert(&mut self, value: T) -> bool {
        // se já contém, não insere
        if self.contains(&value) {
            return false;
        }

        // insere no começo da lista (mais simples)
        let new_node = Box::new(Node {
            elem: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
        true
    }

    fn remove(&mut self, value: &T) -> bool {
        // `cur` é um ponteiro cru para o *link* atual: &mut Option<Box<Node<T>>>
        let mut cur: *mut Option<Box<Node<T>>> = &mut self.head;

        unsafe {
            while let Some(ref mut node) = *cur {
                if node.elem == *value {
                    // pula este nó ligando direto no próximo
                    let next = node.next.take();
                    *cur = next;
                    self.len -= 1;
                    return true;
                } else {
                    // avança: agora o link atual passa a ser o `next`
                    cur = &mut node.next;
                }
            }
        }

        false
    }

    fn contains(&self, value: &T) -> bool {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if &node.elem == value {
                return true;
            }
            cur = &node.next;
        }
        false
    }

    fn len(&self) -> usize {
        self.len
    }
}
