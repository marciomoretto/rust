use super::Sorter;

pub struct HeapSort;

impl<T> Sorter<T> for HeapSort
where
    T: Ord,
{
    fn name() -> &'static str {
        "heap_sort"
    }

    fn sort(slice: &mut [T]) {
        heapsort(slice);
    }
}

// Heapsort in-place usando heap máximo em array 0-based.
fn heapsort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // 1) build max-heap
    build_max_heap(slice);

    // 2) extrai máximo e encolhe o heap
    // posição `end` é o limite exclusivo do heap
    for end in (1..len).rev() {
        slice.swap(0, end);        // move o maior para o final
        heapify_down(slice, 0, end); // restaura heap em [0, end)
    }
}

fn build_max_heap<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // últimos nós internos estão em 0..=(len/2 - 1)
    for idx in (0..=(len / 2)).rev() {
        heapify_down(slice, idx, len);
    }
}

/// "Afunda" o elemento em `idx` até restaurar a propriedade de heap máximo
/// no intervalo [0, len).
fn heapify_down<T: Ord>(slice: &mut [T], mut idx: usize, len: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;

        if left >= len {
            break; // sem filhos
        }

        // maior filho
        let mut largest = left;
        if right < len && slice[right] > slice[left] {
            largest = right;
        }

        if slice[largest] > slice[idx] {
            slice.swap(idx, largest);
            idx = largest;
        } else {
            break;
        }
    }
}
