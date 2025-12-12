use super::Sorter;

pub struct RadixSort;

impl Sorter<i32> for RadixSort {
    fn name() -> &'static str {
        "radix_sort_lsd"
    }

    fn sort(slice: &mut [i32]) {
        radix_lsd(slice);
    }
}

fn radix_lsd(slice: &mut [i32]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }

    let mut output = vec![0i32; n];

    // 4 passes de 8 bits (32 bits)
    for pass in 0..4 {
        let mut counts = [0usize; 256];

        // Conta por byte
        for &v in slice.iter() {
            let key = normalize_key(v);
            let byte = ((key >> (pass * 8)) & 0xFF) as usize;
            counts[byte] += 1;
        }

        // Prefix sums
        for i in 1..256 {
            counts[i] += counts[i - 1];
        }

        // Distribuição estável (varrendo de trás pra frente)
        for &v in slice.iter().rev() {
            let key = normalize_key(v);
            let byte = ((key >> (pass * 8)) & 0xFF) as usize;
            counts[byte] -= 1;
            let pos = counts[byte];
            output[pos] = v;
        }

        // Copia de volta
        slice.copy_from_slice(&output);
    }
}

/// Mapeia i32 para u32 tal que a ordem de i32 seja preservada
/// após a ordenação unsigned: negativos vêm antes dos não-negativos.
#[inline]
fn normalize_key(v: i32) -> u32 {
    (v as u32) ^ 0x8000_0000
}
