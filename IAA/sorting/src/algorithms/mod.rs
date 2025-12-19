pub trait Sorter<T: Ord> {
    fn name() -> &'static str;
    fn sort(slice: &mut [T]);
}

pub mod bubble;
pub mod insertion;
pub mod selection;
pub mod rust;
pub mod merge; 
pub mod quick;
pub mod heap;
pub mod counting;
pub mod radix;
pub mod bucket;

pub use bubble::Bubble;
pub use insertion::Insertion;
pub use selection::Selection;
pub use rust::RustStd;
pub use merge::MergeSort;
pub use quick::QuickSort;
pub use heap::HeapSort;
pub use counting::CountingSort;
pub use radix::RadixSort;
pub use bucket::BucketSort;
