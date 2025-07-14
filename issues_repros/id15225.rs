//ISSUE #15225 <https://github.com/rust-lang/rust-clippy/issues/15225> - C-bug, I-false-positive

#[deny(clippy::arithmetic_side_effects)]
pub fn do_something(v: NonZeroUsize) {
    let _n = v.get() - 1;
    todo!()
}

fn main() {}