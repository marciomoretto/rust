use super::Sorter;

pub struct Insertion;

impl<T: Ord> Sorter<T> for Insertion {
    fn name() -> &'static str {
        "insertion"
    }

    fn sort(slice: &mut [T]) {
        let n = slice.len();
        for i in 1..n {
            let mut j = i;
            while j > 0 && slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
