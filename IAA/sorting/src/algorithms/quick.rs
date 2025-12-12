use super::Sorter;
use rand::prelude::*;

pub struct QuickSort;

impl<T> Sorter<T> for QuickSort
where
    T: Ord,
{
    fn name() -> &'static str {
        "quick"
    }

    fn sort(slice: &mut [T]) {
        quick_sort(slice);
    }
}

/// Quicksort recursivo in-place com pivot aleatório (Lomuto).
fn quick_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    let p = partition_random(slice);

    let (left, right_with_pivot) = slice.split_at_mut(p);
    let (_pivot, right) = right_with_pivot.split_first_mut().unwrap();

    quick_sort(left);
    quick_sort(right);
}

/// Partição de Lomuto com pivot aleatório.
/// O pivot escolhido é movido para o fim antes da partição.
fn partition_random<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();

    // Escolhe um pivot aleatório
    let pivot_index = thread_rng().gen_range(0..len);

    // Move o pivot para o final
    slice.swap(pivot_index, len - 1);

    // Partição Lomuto normal
    partition(slice)
}

/// Partição tradicional de Lomuto com slice[len-1] como pivot.
fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_index = len - 1;

    let mut i = 0;
    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}
