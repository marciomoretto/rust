use super::Sorter;

pub struct CountingSort;

impl Sorter<i32> for CountingSort {
    fn name() -> &'static str {
        "counting_sort"
    }

    fn sort(slice: &mut [i32]) {
        counting_sort(slice);
    }
}

fn counting_sort(slice: &mut [i32]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }

    // Descobre min e max
    let (&min, &max) = match (slice.iter().min(), slice.iter().max()) {
        (Some(mi), Some(ma)) => (mi, ma),
        _ => return,
    };

    // Range de valores (max - min + 1)
    let range = (max as i64 - min as i64 + 1) as usize;

    // Se o range explodir (caso patológico), cai pra sort() da std
    if range > 1_000_000_0 {
        slice.sort();
        return;
    }

    let mut counts = vec![0usize; range];

    // Conta ocorrências
    for &v in slice.iter() {
        let idx = (v - min) as usize;
        counts[idx] += 1;
    }

    // Prefix sums
    for i in 1..range {
        counts[i] += counts[i - 1];
    }

    // Saída estável
    let mut output = vec![0i32; n];
    for &v in slice.iter().rev() {
        let idx = (v - min) as usize;
        counts[idx] -= 1;
        let pos = counts[idx];
        output[pos] = v;
    }

    slice.copy_from_slice(&output);
}
