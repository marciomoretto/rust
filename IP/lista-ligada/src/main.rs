use text_io::read;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Insere no final da lista (para construir a lista inicial)
fn push_back(head: &mut Option<Box<Node>>, data: i32) {
    match head {
        None => {
            *head = Some(Box::new(Node { data, next: None }));
        }
        Some(ref mut node) => {
            let mut cursor = node;
            while let Some(ref mut next) = cursor.next {
                cursor = next;
            }
            cursor.next = Some(Box::new(Node { data, next: None }));
        }
    }
}

// Função pedida no exercício
fn insert_at_position(
    head: Option<Box<Node>>,
    data: i32,
    position: usize,
) -> Option<Box<Node>> {
    // Caso especial: inserir na posição 0 (começo da lista)
    if position == 0 {
        return Some(Box::new(Node { data, next: head }));
    }

    // Vamos caminhar até o nó anterior à posição
    let mut current = head;
    let mut cursor = &mut current;

    for _ in 0..position - 1 {
        if let Some(ref mut node) = cursor {
            cursor = &mut node.next;
        } else {
            // posição inválida, mas pelo enunciado 0 ≤ position ≤ n, então não deveria acontecer
            break;
        }
    }

    if let Some(ref mut node) = cursor {
        let old_next = node.next.take(); // guarda o resto da lista
        node.next = Some(Box::new(Node {
            data,
            next: old_next,
        }));
    }

    current
}

fn print_list(mut head: &Option<Box<Node>>) {
    let mut first = true;
    while let Some(node) = head {
        if !first {
            print!(" ");
        }
        print!("{}", node.data);
        first = false;
        head = &node.next;
    }
    println!();
}

fn main() {
    // n: número de elementos da lista
    let n: usize = read!();

    // lê os n elementos e monta a lista
    let mut head: Option<Box<Node>> = None;
    for _ in 0..n {
        let value: i32 = read!();
        push_back(&mut head, value);
    }

    // lê o dado e a posição para inserção
    let data: i32 = read!();
    let position: usize = read!();

    head = insert_at_position(head, data, position);

    print_list(&head);
}
