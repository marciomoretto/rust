pub mod linear;
pub mod binary;

pub type SearchFn = fn(&[i32], i32) -> bool;

#[derive(Clone, Copy)]
pub struct Implementation {
    pub name: &'static str,
    pub f: SearchFn,
}

pub const IMPLEMENTATIONS: &[Implementation] = &[
    Implementation { name: "linear", f: linear::search_linear },
    Implementation { name: "binary", f: binary::search_binary },
];
