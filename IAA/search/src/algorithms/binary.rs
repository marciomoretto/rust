use std::hint::black_box;

#[inline(never)]
pub fn search_binary(a: &[i32], x: i32) -> bool {
    if a.is_empty() {
        return false;
    }
    search_binary_rec(a, x, 0, a.len() as isize - 1)
}

#[inline(never)]
fn search_binary_rec(a: &[i32], x: i32, l: isize, r: isize) -> bool {
    if l > r {
        return false;
    }

    let m = l + ((r - l) / 2);
    let v = black_box(a[m as usize]); 
    if v == x {
        true
    } else if x < v {
        search_binary_rec(a, x, l, m - 1)
    } else {
        search_binary_rec(a, x, m + 1, r)
    }
}