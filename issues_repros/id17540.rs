//ISSUE #17540 <https://github.com/rust-lang/rust-clippy/issues/17540> - C-bug, I-false-negative

#![deny(clippy::arithmetic_side_effects)]

fn foo(n: u32) -> u32 {
    n.pow(20)
}

fn main() {}