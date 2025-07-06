//ISSUE #15195 <https://github.com/rust-lang/rust-clippy/issues/15195> - C-bug, I-suggestion-causes-error
fn main() {}

#[deny(clippy::expect_fun_call)]
fn f(x: Option<i32>) {
    let s = String::from("foo");
    let s = s.as_ref();
    _ = x.expect(std::convert::identity(s));
    println!("s = {s}");
}
