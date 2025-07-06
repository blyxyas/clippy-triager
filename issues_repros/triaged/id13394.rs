//ISSUE #13394 - C-bug, I-false-positive

#![warn(clippy::missing_inline_in_public_items)]
fn main() {}

#[expect(clippy::missing_inline_in_public_items)]
pub fn foo() -> u32 {
    0
}
