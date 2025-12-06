/// Erros possíveis ao operar sobre a fila.
#[derive(Debug, PartialEq, Eq)]
pub enum QueueError {
    /// Tentativa de enfileirar em uma fila cheia.
    Overflow,
}

/// Trait que define o TAD Fila.
pub trait Queue<T> {
    fn enqueue(&mut self, value: T) -> Result<(), QueueError>;

    fn dequeue(&mut self) -> Option<T>;

    /// Olha o elemento do início da fila, sem remover.
    fn front(&self) -> Option<&T>;

    /// Retorna `true` se a fila está vazia.
    fn is_empty(&self) -> bool;
}
