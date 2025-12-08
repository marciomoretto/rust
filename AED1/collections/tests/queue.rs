use collections::queue::Queue;
use collections::queue::{ArrayQueue, ListQueue};

/// Função auxiliar: testa comportamento FIFO básico em qualquer implementação de Queue<i32>.
fn testa_fila_basica<Q: Queue<i32>>(mut q: Q) {
    assert!(q.is_empty());

    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);

    // FIFO: primeiro a entrar, primeiro a sair
    assert_eq!(q.front(), Some(&10));

    assert_eq!(q.dequeue(), Some(10));
    assert_eq!(q.dequeue(), Some(20));
    assert_eq!(q.dequeue(), Some(30));
    assert_eq!(q.dequeue(), None);
    assert!(q.is_empty());
}

#[test]
fn arrayqueue_nova_fila_deve_comecar_vazia() {
    let q: ArrayQueue<i32> = ArrayQueue::new();
    assert!(q.is_empty());
}

#[test]
fn arrayqueue_deve_seguir_ordem_fifo() {
    let q: ArrayQueue<i32> = ArrayQueue::new();
    testa_fila_basica(q);
}

#[test]
fn arrayqueue_front_nao_deve_remover_o_elemento() {
    let mut q: ArrayQueue<i32> = ArrayQueue::new();

    q.enqueue(1);
    q.enqueue(2);

    assert_eq!(q.front(), Some(&1));

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), None);
}

#[test]
fn arrayqueue_deve_funcionar_com_tipo_nao_copy() {
    let mut q: ArrayQueue<String> = ArrayQueue::new();

    q.enqueue("oi".to_string());
    q.enqueue("tchau".to_string());

    assert_eq!(q.front().map(|s| s.as_str()), Some("oi"));

    assert_eq!(q.dequeue().as_deref(), Some("oi"));
    assert_eq!(q.dequeue().as_deref(), Some("tchau"));
    assert_eq!(q.dequeue(), None);
}

// Agora repetimos os mesmos testes conceituais para a ListQueue:

#[test]
fn listqueue_nova_fila_deve_comecar_vazia() {
    let q: ListQueue<i32> = ListQueue::new();
    assert!(q.is_empty());
}

#[test]
fn listqueue_deve_seguir_ordem_fifo() {
    let q: ListQueue<i32> = ListQueue::new();
    testa_fila_basica(q);
}

#[test]
fn listqueue_front_nao_deve_remover_o_elemento() {
    let mut q: ListQueue<i32> = ListQueue::new();

    q.enqueue(1);
    q.enqueue(2);

    assert_eq!(q.front(), Some(&1));

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), None);
}

#[test]
fn listqueue_deve_funcionar_com_tipo_nao_copy() {
    let mut q: ListQueue<String> = ListQueue::new();

    q.enqueue("oi".to_string());
    q.enqueue("tchau".to_string());

    assert_eq!(q.front().map(|s| s.as_str()), Some("oi"));

    assert_eq!(q.dequeue().as_deref(), Some("oi"));
    assert_eq!(q.dequeue().as_deref(), Some("tchau"));
    assert_eq!(q.dequeue(), None);
}
