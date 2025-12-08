pub trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;

    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}

pub mod array_stack;
pub mod list_stack;

pub use array_stack::ArrayStack;
pub use list_stack::ListStack;
