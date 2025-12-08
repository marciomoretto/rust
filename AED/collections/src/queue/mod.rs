/// Trait que define o TAD Fila.
pub trait Queue<T> {
    fn enqueue(&mut self, value: T);

    fn dequeue(&mut self) -> Option<T>;

    /// Olha o elemento do início da fila, sem remover.
    fn front(&self) -> Option<&T>;

    /// Retorna `true` se a fila está vazia.
    fn is_empty(&self) -> bool {
        self.front().is_none()
    }
}

pub mod array_queue;
pub mod list_queue;

pub use array_queue::ArrayQueue;
pub use list_queue::ListQueue;
