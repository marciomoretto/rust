use super::DuplicateChecker;

pub struct LinearSeen;

impl DuplicateChecker<usize> for LinearSeen {
    fn name() -> &'static str {
        "linear_seen"
    }

    fn has_duplicate(slice: &[usize]) -> bool {
        let n = slice.len();

        // vetor visto[0..n-1] inicializado com false
        let mut seen = vec![false; n];

        for &x in slice {
            // assumimos que x está em 0..n-1
            if seen[x] {
                return true;
            }
            seen[x] = true;
        }

        false
    }
}
