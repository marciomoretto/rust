use crate::set::Set;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    height: i32,
}

/// Conjunto implementado como árvore AVL.
/// Não permite elementos repetidos.
/// Operações são O(log n) em média.
pub struct BstAvlSet<T> {
    root: Link<T>,
    len: usize,
}

impl<T> BstAvlSet<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

// ========================
// Helpers de AVL
// ========================

fn height<T>(link: &Link<T>) -> i32 {
    match link {
        Some(node) => node.height,
        None => 0,
    }
}

fn update_height<T>(node: &mut Box<Node<T>>) {
    let hl = height(&node.left);
    let hr = height(&node.right);
    node.height = 1 + hl.max(hr);
}

fn balance_factor<T>(node: &Box<Node<T>>) -> i32 {
    height(&node.left) - height(&node.right)
}

/// Rotação simples à direita em `root`
fn rotate_right<T>(root: &mut Link<T>) {
    let mut x = root.take().expect("rotate_right: root vazio");
    let mut y = x
        .left
        .take()
        .expect("rotate_right: nó sem filho esquerdo");

    // y.right sobe para x.left
    x.left = y.right.take();

    // atualiza alturas de baixo pra cima
    update_height(&mut x);
    update_height(&mut y);

    // x vira filho direito de y
    y.right = Some(x);
    *root = Some(y);
}

/// Rotação simples à esquerda em `root`
fn rotate_left<T>(root: &mut Link<T>) {
    let mut x = root.take().expect("rotate_left: root vazio");
    let mut y = x
        .right
        .take()
        .expect("rotate_left: nó sem filho direito");

    // y.left sobe para x.right
    x.right = y.left.take();

    // atualiza alturas
    update_height(&mut x);
    update_height(&mut y);

    // x vira filho esquerdo de y
    y.left = Some(x);
    *root = Some(y);
}

/// Rebalanceia a subárvore enraizada em `root`,
/// assumindo que as alturas dos filhos estão corretas.
fn rebalance<T>(root: &mut Link<T>) {
    if let Some(node) = root.as_mut() {
        update_height(node);
        let bf = balance_factor(node);

        // árvore pesada à esquerda
        if bf > 1 {
            if let Some(left) = node.left.as_ref() {
                // caso LR (Left-Right): primeiro rotaciona filho para a esquerda
                if balance_factor(left) < 0 {
                    rotate_left(&mut node.left);
                }
            }
            rotate_right(root);
        }
        // árvore pesada à direita
        else if bf < -1 {
            if let Some(right) = node.right.as_ref() {
                // caso RL (Right-Left): primeiro rotaciona filho para a direita
                if balance_factor(right) > 0 {
                    rotate_right(&mut node.right);
                }
            }
            rotate_left(root);
        }
    }
}

// Remove o nó com menor elemento da subárvore `link`
// e devolve esse nó (como Box<Node<T>>).
fn take_min_node<T: Ord>(link: &mut Link<T>) -> Box<Node<T>> {
    // pré-condição: link é Some
    if link.as_ref().unwrap().left.is_none() {
        // este é o mínimo
        let mut boxed = link.take().unwrap();
        // o filho direito sobe
        *link = boxed.right.take();
        boxed
    } else {
        // desce pela esquerda
        let min_node = {
            let left_link = &mut link.as_mut().unwrap().left;
            take_min_node(left_link)
        };
        // após a remoção no filho esquerdo, rebalanceia esta raiz
        rebalance(link);
        min_node
    }
}

// Remove a raiz da subárvore apontada por `link`
// (sabendo que ela EXISTE) e ajusta `len`.
fn remove_at_root<T: Ord>(link: &mut Link<T>, len: &mut usize) {
    let mut node = link.take().unwrap();
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
        (Some(left), Some(right_root)) => {
            // dois filhos:
            // - pegamos o menor nó da subárvore direita
            // - esse nó vira a nova raiz desta subárvore
            let mut right_link = Some(right_root);
            let mut min_node = take_min_node(&mut right_link);

            min_node.left = Some(left);
            min_node.right = right_link;

            update_height(&mut min_node);
            *link = Some(min_node);
        }
    }

    if link.is_some() {
        rebalance(link);
    }
}

// ========================
// Implementação de Set<T>
// ========================

impl<T: Ord> Set<T> for BstAvlSet<T> {
    fn insert(&mut self, value: T) -> bool {
        fn insert_rec<T: Ord>(link: &mut Link<T>, value: T, len: &mut usize) -> bool {
            match link {
                Some(node) => {
                    let changed = if value < node.elem {
                        insert_rec(&mut node.left, value, len)
                    } else if value > node.elem {
                        insert_rec(&mut node.right, value, len)
                    } else {
                        // já existe, não insere
                        false
                    };

                    if changed {
                        rebalance(link);
                    }

                    changed
                }
                None => {
                    *link = Some(Box::new(Node {
                        elem: value,
                        left: None,
                        right: None,
                        height: 1,
                    }));
                    *len += 1;
                    true
                }
            }
        }

        insert_rec(&mut self.root, value, &mut self.len)
    }

    fn remove(&mut self, value: &T) -> bool {
        fn remove_rec<T: Ord>(link: &mut Link<T>, value: &T, len: &mut usize) -> bool {
            match link {
                None => false,
                Some(node) => {
                    if value < &node.elem {
                        let removed = remove_rec(&mut node.left, value, len);
                        if removed {
                            rebalance(link);
                        }
                        removed
                    } else if value > &node.elem {
                        let removed = remove_rec(&mut node.right, value, len);
                        if removed {
                            rebalance(link);
                        }
                        removed
                    } else {
                        // achamos o nó a remover na raiz desta subárvore
                        remove_at_root(link, len);
                        true
                    }
                }
            }
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
