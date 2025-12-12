use super::Sorter;

pub struct Bubble;

impl<T: Ord> Sorter<T> for Bubble {
    fn name() -> &'static str {
        "bubble"
    }

    fn sort(slice: &mut [T]) {
        let n = slice.len();
        if n <= 1 {
            return;
        }

        for i in 0..n {
            let mut swapped = false;
            for j in 0..(n - 1 - i) {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}
