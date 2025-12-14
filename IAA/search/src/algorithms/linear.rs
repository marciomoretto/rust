use std::hint::black_box;

#[inline(never)]
pub fn search_linear(a: &[i32], x: i32) -> bool {
    for &v in a {
        let v = black_box(v);
        if v == x {
            return true;
        }
    }
    false
}
