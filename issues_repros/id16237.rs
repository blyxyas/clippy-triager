//ISSUE #16237 <https://github.com/rust-lang/rust-clippy/issues/16237> - C-bug, I-false-positive

#![warn(clippy::missing_asserts_for_indexing)]

const MIN_LEN: usize = 3;
pub fn test(s: &[u8]) {
    assert!(s.len() >= MIN_LEN);
    s[0];
    s[1];
}

fn main() {}