use super::Sorter;

pub struct Selection;

impl<T: Ord> Sorter<T> for Selection {
    fn name() -> &'static str {
        "selection"
    }

    fn sort(slice: &mut [T]) {
        let n = slice.len();
        for i in 0..n {
            let mut min = i;
            for j in (i + 1)..n {
                if slice[j] < slice[min] {
                    min = j;
                }
            }
            slice.swap(i, min);
        }
    }
}
