use crate::set::Set;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Black,
}

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    color: Color, // cor do link que chega neste nó
}

/// Conjunto implementado como red-black tree left-leaning (Sedgewick).
/// Não permite elementos repetidos.
/// Operações são O(log n).
pub struct BstRBSet<T> {
    root: Link<T>,
    len: usize,
}

impl<T> BstRBSet<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// ========================
// Helpers de cor / rotações
// ========================

fn is_red_link<T>(link: &Link<T>) -> bool {
    matches!(link, Some(node) if node.color == Color::Red)
}

// Versão em Box, mais próxima do estilo do Sedgewick (retorna nova raiz da subárvore)

fn rotate_left_box<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    // supõe h.right é Some
    let mut x = h.right.take().expect("rotate_left_box: sem filho direito");
    h.right = x.left.take();
    x.color = h.color;
    h.color = Color::Red;
    x.left = Some(h);
    x
}

fn rotate_right_box<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    // supõe h.left é Some
    let mut x = h.left.take().expect("rotate_right_box: sem filho esquerdo");
    h.left = x.right.take();
    x.color = h.color;
    h.color = Color::Red;
    x.right = Some(h);
    x
}

fn flip_colors_box<T>(h: &mut Box<Node<T>>) {
    h.color = match h.color {
        Color::Red => Color::Black,
        Color::Black => Color::Red,
    };

    if let Some(ref mut left) = h.left {
        left.color = match left.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
    if let Some(ref mut right) = h.right {
        right.color = match right.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
}

/// Fix-up de uma subárvore, na mesma ordem do Sedgewick:
/// 1) corrige vermelho à direita
/// 2) corrige dois vermelhos seguidos à esquerda
/// 3) trata nó-4 (dois filhos vermelhos)
fn fix_up_box<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    // 1) link vermelho à direita e não à esquerda → rotateLeft
    if is_red_link(&h.right) && !is_red_link(&h.left) {
        h = rotate_left_box(h);
    }

    // 2) dois vermelhos seguidos à esquerda → rotateRight
    if is_red_link(&h.left) {
        let left_left_red = if let Some(ref left) = h.left {
            is_red_link(&left.left)
        } else {
            false
        };

        if left_left_red {
            h = rotate_right_box(h);
        }
    }

    // 3) ambos filhos vermelhos → flipColors
    if is_red_link(&h.left) && is_red_link(&h.right) {
        flip_colors_box(&mut h);
    }

    h
}

// ========================
// moveRedLeft / moveRedRight (remoção)
// ========================

fn move_red_left_box<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    // pressupõe que h é preto e tanto h.left quanto h.left.left são pretos:
    // empresta vermelho para a esquerda.
    flip_colors_box(&mut h);

    let right_left_red = if let Some(ref right) = h.right {
        is_red_link(&right.left)
    } else {
        false
    };

    if right_left_red {
        if let Some(right_box) = h.right.take() {
            h.right = Some(rotate_right_box(right_box));
        }
        h = rotate_left_box(h);
        flip_colors_box(&mut h);
    }

    h
}

fn move_red_right_box<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    // análogo: empresta vermelho para a direita
    flip_colors_box(&mut h);

    let left_left_red = if let Some(ref left) = h.left {
        is_red_link(&left.left)
    } else {
        false
    };

    if left_left_red {
        h = rotate_right_box(h);
        flip_colors_box(&mut h);
    }

    h
}

// ========================
// deleteMin (usado dentro de delete)
// ========================

fn delete_min_box<T: Ord>(mut h: Box<Node<T>>) -> (T, Link<T>) {
    if h.left.is_none() {
        let elem = h.elem;
        // a subárvore resultante é apenas o filho direito
        return (elem, h.right.take());
    }

    if !is_red_link(&h.left) {
        let left_left_red = if let Some(ref left) = h.left {
            is_red_link(&left.left)
        } else {
            false
        };

        if !left_left_red {
            h = move_red_left_box(h);
        }
    }

    let left = h.left.take().unwrap();
    let (min, new_left) = delete_min_box(left);
    h.left = new_left;
    let h = fix_up_box(h);
    (min, Some(h))
}

// ========================
// Inserção (put) estilo Sedgewick
// ========================

fn insert_rec<T: Ord>(h: Link<T>, value: T, inserted: &mut bool) -> Link<T> {
    match h {
        None => {
            *inserted = true;
            return Some(Box::new(Node {
                elem: value,
                left: None,
                right: None,
                color: Color::Red,
            }));
        }

        Some(mut node) => {
            if value < node.elem {
                node.left = insert_rec(node.left, value, inserted);
            } else if value > node.elem {
                node.right = insert_rec(node.right, value, inserted);
            } else {
                // já existe — não insere e não muda cor nem estrutura
                return Some(node);
            }

            // === Fix-up, exatamente como o Sedgewick ===

            if is_red_link(&node.right) && !is_red_link(&node.left) {
                node = rotate_left_box(node);
            }

            if is_red_link(&node.left) {
                if let Some(ref left) = node.left {
                    if is_red_link(&left.left) {
                        node = rotate_right_box(node);
                    }
                }
            }

            if is_red_link(&node.left) && is_red_link(&node.right) {
                flip_colors_box(&mut node);
            }

            return Some(node);
        }
    }
}


// ========================
// Remoção (delete) estilo Sedgewick
// ========================

fn delete_rec<T: Ord>(h: Link<T>, key: &T, removed: &mut bool) -> Link<T> {
    let mut h = match h {
        None => return None,
        Some(node) => node,
    };

    if *key < h.elem {
        // vamos descer pela esquerda; garante vermelho disponível à esquerda
        if !is_red_link(&h.left) {
            let left_left_red = if let Some(ref left) = h.left {
                is_red_link(&left.left)
            } else {
                false
            };
            if !left_left_red {
                h = move_red_left_box(h);
            }
        }
        h.left = delete_rec(h.left.take(), key, removed);
    } else {
        // se o filho esquerdo é vermelho, rotaciona para trazer a chave para baixo
        if is_red_link(&h.left) {
            h = rotate_right_box(h);
        }

        // caso: achou a chave num nó sem filho direito → remover folha
        if *key == h.elem && h.right.is_none() {
            *removed = true;
            return None;
        }

        // vamos descer à direita; garante vermelho disponível à direita
        if !is_red_link(&h.right) {
            let right_left_red = if let Some(ref right) = h.right {
                is_red_link(&right.left)
            } else {
                false
            };
            if !right_left_red {
                h = move_red_right_box(h);
            }
        }

        if *key == h.elem {
            // substitui pela menor chave da subárvore direita
            let right = h.right.take().unwrap();
            let (min_key, new_right) = delete_min_box(right);
            h.elem = min_key;
            h.right = new_right;
            *removed = true;
        } else {
            h.right = delete_rec(h.right.take(), key, removed);
        }
    }

    Some(fix_up_box(h))
}

// ========================
// Implementação de Set<T>
// ========================

impl<T: Ord> Set<T> for BstRBSet<T> {
    fn insert(&mut self, value: T) -> bool {
        let mut inserted = false;
        self.root = insert_rec(self.root.take(), value, &mut inserted);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black; // raiz sempre preta
        }
        if inserted {
            self.len += 1;
        }
        inserted
    }

    fn remove(&mut self, value: &T) -> bool {
        if self.root.is_none() {
            return false;
        }

        // opcional: se ambos filhos da raiz são pretos, torna raiz vermelha
        // (padrão do Sedgewick para simplificar lógica)
        if !is_red_link(&self.root.as_ref().unwrap().left)
            && !is_red_link(&self.root.as_ref().unwrap().right)
        {
            if let Some(ref mut root) = self.root {
                root.color = Color::Red;
            }
        }

        let mut removed = false;
        self.root = delete_rec(self.root.take(), value, &mut removed);

        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }

        if removed {
            self.len -= 1;
        }

        removed
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

impl<T> crate::set::SetName for BstRBSet<T> {
    fn name() -> &'static str {
        "BstRBSet"
    }
}

impl<T> Default for BstRBSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
