use super::DuplicateChecker;

pub struct Naive;

impl<T: Eq> DuplicateChecker<T> for Naive {
    fn name() -> &'static str {
        "naive"
    }

    fn has_duplicate(slice: &[T]) -> bool {
        let n = slice.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if slice[i] == slice[j] {
                    return true;
                }
            }
        }
        false
    }
}
