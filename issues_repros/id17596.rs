//ISSUE #17596 <https://github.com/rust-lang/rust-clippy/issues/17596> - C-bug, I-suggestion-causes-error, I-false-positive

#![feature(try_blocks)]
#![deny(clippy::if_then_some_else_none)]

fn foo(x: u32) -> Option<Option<u32>> {
    try {
        if x > 0 {
            Some(x.checked_sub(2)?)
        } else {
            None
        }
    }
}

fn main() {}