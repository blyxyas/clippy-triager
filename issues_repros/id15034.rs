//ISSUE #15034 - C-bug, I-false-negative
fn main() {}

struct S;
impl S {
    #[expect(clippy::unnecessary_safety_comment)]
    /// SAFETY: ...
    fn f() {}
}
