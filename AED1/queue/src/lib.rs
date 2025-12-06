pub mod tad;        // pilha – trait + erro
pub mod array;      // pilha – implementação com arranjo
pub mod list;       // pilha – implementação com lista ligada

pub use tad::{Queue, QueueError};
pub use array::ArrayQueue;
pub use list::ListQueue;
