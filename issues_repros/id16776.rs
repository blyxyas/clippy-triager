//ISSUE #16776 <https://github.com/rust-lang/rust-clippy/issues/16776> - C-bug, I-false-negative

fn maunal_filter_and_then(v: Option<usize>) -> Option<usize> {
    match v {
        Some(n) => {
            if n > 10 {
                Some(n + 1)
            } else {
                None
            }
        }
        None => None,
    }
}

fn main() {}