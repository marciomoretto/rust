use super::Sorter;

pub struct RustStd;

impl<T: Ord> Sorter<T> for RustStd {
    fn name() -> &'static str {
        "rust_std_sort"
    }

    fn sort(slice: &mut [T]) {
        slice.sort();
    }
}
