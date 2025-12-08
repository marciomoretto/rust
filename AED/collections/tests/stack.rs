use collections::stack::{ArrayStack, ListStack, Stack};

/// Função auxiliar que testa o comportamento básico LIFO
/// para qualquer implementação de Stack<i32>.
fn testa_pilha_basica<S: Stack<i32>>(mut s: S) {
    assert!(s.is_empty());

    s.push(10);
    s.push(20);
    s.push(30);

    assert_eq!(s.peek(), Some(&30));

    assert_eq!(s.pop(), Some(30));
    assert_eq!(s.pop(), Some(20));
    assert_eq!(s.pop(), Some(10));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}

#[test]
fn arraystack_nova_pilha_deve_comecar_vazia() {
    let s: ArrayStack<i32> = ArrayStack::new();
    assert!(s.is_empty());
}

#[test]
fn arraystack_push_e_pop_devem_seguir_ordem_lifo() {
    let s: ArrayStack<i32> = ArrayStack::new();
    testa_pilha_basica(s);
}

#[test]
fn arraystack_peek_nao_deve_remover_o_elemento() {
    let mut s: ArrayStack<i32> = ArrayStack::new();

    s.push(1);
    s.push(2);

    assert_eq!(s.peek(), Some(&2));

    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

// não existe mais overflow: ArrayStack cresce com o ResizableArray,
// então removemos o teste de overflow.

#[test]
fn arraystack_deve_funcionar_com_tipo_nao_copy() {
    let mut s: ArrayStack<String> = ArrayStack::new();

    s.push("oi".to_string());
    s.push("tchau".to_string());

    assert_eq!(s.peek().map(|s| s.as_str()), Some("tchau"));

    assert_eq!(s.pop().as_deref(), Some("tchau"));
    assert_eq!(s.pop().as_deref(), Some("oi"));
    assert_eq!(s.pop(), None);
}

// Agora repetimos os mesmos testes conceituais para a ListStack:

#[test]
fn liststack_nova_pilha_deve_comecar_vazia() {
    let s: ListStack<i32> = ListStack::new();
    assert!(s.is_empty());
}

#[test]
fn liststack_push_e_pop_devem_seguir_ordem_lifo() {
    let s: ListStack<i32> = ListStack::new();
    testa_pilha_basica(s);
}

#[test]
fn liststack_peek_nao_deve_remover_o_elemento() {
    let mut s: ListStack<i32> = ListStack::new();

    s.push(1);
    s.push(2);

    assert_eq!(s.peek(), Some(&2));

    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn liststack_deve_funcionar_com_tipo_nao_copy() {
    let mut s: ListStack<String> = ListStack::new();

    s.push("oi".to_string());
    s.push("tchau".to_string());

    assert_eq!(s.peek().map(|s| s.as_str()), Some("tchau"));

    assert_eq!(s.pop().as_deref(), Some("tchau"));
    assert_eq!(s.pop().as_deref(), Some("oi"));
    assert_eq!(s.pop(), None);
}
