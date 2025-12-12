use super::Sorter;

pub struct MergeSort;

impl<T> Sorter<T> for MergeSort
where
    T: Ord + Clone,
{
    fn name() -> &'static str {
        "merge_sort_td"
    }

    fn sort(slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }

        // buffer temporário para o merge
        let mut buffer: Vec<T> = slice.to_vec();
        merge_sort_recursive(slice, &mut buffer, 0, len);
    }
}

/// Ordena slice[l..r) usando buffer como área temporária.
/// Estratégia: divide recursivamente e faz merge estável.
fn merge_sort_recursive<T>(slice: &mut [T], buffer: &mut [T], l: usize, r: usize)
where
    T: Ord + Clone,
{
    if r - l <= 1 {
        return;
    }

    let m = (l + r) / 2;
    merge_sort_recursive(slice, buffer, l, m);
    merge_sort_recursive(slice, buffer, m, r);

    // merge estável de [l, m) e [m, r) em buffer, depois copia de volta
    let mut i = l;
    let mut j = m;
    let mut k = l;

    while i < m && j < r {
        if slice[i] <= slice[j] {
            buffer[k] = slice[i].clone();
            i += 1;
        } else {
            buffer[k] = slice[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < m {
        buffer[k] = slice[i].clone();
        i += 1;
        k += 1;
    }

    while j < r {
        buffer[k] = slice[j].clone();
        j += 1;
        k += 1;
    }

    // copia de volta pro slice
    slice[l..r].clone_from_slice(&buffer[l..r]);
}
