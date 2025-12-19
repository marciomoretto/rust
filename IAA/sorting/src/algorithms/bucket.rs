use super::{Insertion, Sorter};
use ordered_float::OrderedFloat;

pub struct BucketSort;

impl Sorter<OrderedFloat<f64>> for BucketSort {
    fn name() -> &'static str {
        "bucket_sort"
    }

    fn sort(slice: &mut [OrderedFloat<f64>]) {
        bucket_sort(slice);
    }
}

fn bucket_sort(slice: &mut [OrderedFloat<f64>]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }

    // min/max (OrderedFloat tem Ord)
    let (&min, &max) = match (slice.iter().min(), slice.iter().max()) {
        (Some(mi), Some(ma)) => (mi, ma),
        _ => return,
    };

    if min == max {
        return;
    }

    let minv = min.0;
    let maxv = max.0;

    // Normalização para índice de balde:
    // idx = floor(n * (x - min)/(max-min+1))
    // (+1 evita cair exatamente em 1.0 por arredondamento quando x = max)
    let denom = (maxv - minv) + 1.0;

    let mut buckets: Vec<Vec<OrderedFloat<f64>>> = (0..n).map(|_| Vec::new()).collect();

    for &x in slice.iter() {
        let num = x.0 - minv; // >= 0
        let mut idx = ((n as f64) * (num / denom)) as usize;
        if idx >= n {
            idx = n - 1;
        }
        buckets[idx].push(x);
    }

    // Reutiliza seu Insertion genérico
    for b in buckets.iter_mut() {
        Insertion::sort(b.as_mut_slice());
    }

    // Concatena
    let mut out = 0usize;
    for b in buckets.into_iter() {
        slice[out..out + b.len()].copy_from_slice(&b);
        out += b.len();
    }
}
