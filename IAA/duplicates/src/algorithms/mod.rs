pub trait DuplicateChecker<T> {
    fn name() -> &'static str;
    fn has_duplicate(slice: &[T]) -> bool;
}

pub mod naive;
pub mod linear_seen;

pub use naive::Naive;
pub use linear_seen::LinearSeen;
