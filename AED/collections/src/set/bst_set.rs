use crate::set::Set;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

/// Conjunto implementado como árvore binária de busca (não balanceada).
/// Não permite elementos repetidos.
/// Operações são O(altura).
pub struct BstSet<T> {
    root: Link<T>,
    len: usize,
}

impl<T> BstSet<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

// ========================
// Implementação de Set<T>
// ========================

impl<T: Ord> Set<T> for BstSet<T> {
    fn insert(&mut self, value: T) -> bool {
        fn insert_rec<T: Ord>(link: &mut Link<T>, value: T, len: &mut usize) -> bool {
            match link {
                Some(node) => {
                    if value < node.elem {
                        insert_rec(&mut node.left, value, len)
                    } else if value > node.elem {
                        insert_rec(&mut node.right, value, len)
                    } else {
                        // já existe, não insere
                        false
                    }
                }
                None => {
                    *link = Some(Box::new(Node {
                        elem: value,
                        left: None,
                        right: None,
                    }));
                    *len += 1;
                    true
                }
            }
        }

        insert_rec(&mut self.root, value, &mut self.len)
    }

    fn remove(&mut self, value: &T) -> bool {
        // remove o menor elemento da subárvore apontada por `link`
        // e devolve o valor removido.
        fn take_min<T>(link: &mut Link<T>) -> T {
            let mut cur = link;

            loop {
                // enquanto houver filho à esquerda, desce para a esquerda
                if cur.as_ref()
                    .and_then(|node| node.left.as_ref())
                    .is_some()
                {
                    cur = &mut cur.as_mut().unwrap().left;
                    continue;
                }

                // aqui `cur` aponta para o mínimo: não tem filho esquerdo
                let mut boxed = cur.take().expect("take_min chamado em subárvore vazia");
                let right = boxed.right.take(); // filho direito sobe
                *cur = right;
                return boxed.elem;
            }
        }

        fn remove_rec<T: Ord>(link: &mut Link<T>, value: &T, len: &mut usize) -> bool {
            let node = match link {
                Some(node) => node,
                None => return false,
            };

            if value < &node.elem {
                return remove_rec(&mut node.left, value, len);
            } else if value > &node.elem {
                return remove_rec(&mut node.right, value, len);
            }

            // encontramos o nó a ser removido
            *len -= 1;

            match (node.left.take(), node.right.take()) {
                (None, None) => {
                    // folha
                    *link = None;
                }
                (Some(left), None) => {
                    // só filho esquerdo
                    *link = Some(left);
                }
                (None, Some(right)) => {
                    // só filho direito
                    *link = Some(right);
                }
                (Some(left), Some(right_box)) => {
                    // dois filhos:
                    // - pegamos o sucessor em ordem (mínimo da subárvore direita)
                    // - esse valor vira o novo elem da raiz desta subárvore
                    // - e o restante da subárvore direita (sem o mínimo) fica em `right`
                    let mut right: Link<T> = Some(right_box);
                    let succ = take_min(&mut right);

                    *link = Some(Box::new(Node {
                        elem: succ,
                        left: Some(left),
                        right,
                    }));
                }
            }

            true
        }

        remove_rec(&mut self.root, value, &mut self.len)
    }

    fn contains(&self, value: &T) -> bool {
        let mut cur = &self.root;

        while let Some(node) = cur {
            if *value < node.elem {
                cur = &node.left;
            } else if *value > node.elem {
                cur = &node.right;
            } else {
                return true;
            }
        }

        false
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<T> crate::set::SetName for BstSet<T> {
    fn name() -> &'static str {
        "BstSet"
    }
}

impl<T> Default for BstSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
