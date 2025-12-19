use std::hint::black_box;

use super::Sorter;

pub struct RadixSort;

impl Sorter<i32> for RadixSort {
    fn name() -> &'static str {
        "radix_sort"
    }

    fn sort(slice: &mut [i32]) {
        radix(slice);
    }
}

const B: usize = 256;
const D: usize = 4;

#[inline(never)]
fn radix(a: &mut [i32]) {
    let n = a.len();
    if n <= 1 {
        return;
    }

    // buffer auxiliar (B[1..n])
    let mut out = vec![0i32; n];

    // 4 passes (um por byte)
    for pass in 0..D {
        // contadores (C[0..255])
        let mut counts = [0usize; B];

        // 1) contagem
        for &v in a.iter() {
            let key = normalize_key(v);
            let byte = byte_at(key, pass);
            counts[byte] += 1;
        }

        // 2) somas prefixadas
        for i in 1..B {
            counts[i] += counts[i - 1];
        }

        // 3) distribuição estável (de trás pra frente)
        for &v in a.iter().rev() {
            let key = normalize_key(v);
            let byte = byte_at(key, pass);
            counts[byte] -= 1;
            out[counts[byte]] = v;
        }

        // 4) copia de volta
        a.copy_from_slice(&out);

        // “blindagem” mínima p/ benchmark
        black_box(&a);
    }

    black_box(&a);
}

/// Extrai o `pass`-ésimo byte (0 = LSB) de uma chave u32.
#[inline(always)]
fn byte_at(key: u32, pass: usize) -> usize {
    ((key >> (pass * 8)) & 0xFF) as usize
}

/// Normaliza i32 -> u32 preservando a ordem dos inteiros com sinal
/// quando comparados como unsigned (negativos antes dos não-negativos).
#[inline(always)]
fn normalize_key(v: i32) -> u32 {
    (v as u32) ^ 0x8000_0000
}
