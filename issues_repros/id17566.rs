//ISSUE #17566 <https://github.com/rust-lang/rust-clippy/issues/17566> - C-bug, I-false-positive

use std::cell::Cell;
thread_local! {
    static ALREADY_CONST: Cell<Option<u8>> = const { Cell::new(None) };
}
fn main() { ALREADY_CONST.with(|c| c.get()); }

