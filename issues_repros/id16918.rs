//ISSUE #16918 <https://github.com/rust-lang/rust-clippy/issues/16918> - C-bug, L-suggestion

pub fn foo(x: usize) -> () {
    let _ = x;
    ()
}

fn main() {}