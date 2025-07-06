//ISSUE #15142 <https://github.com/rust-lang/rust-clippy/issues/15142> - C-bug, I-suggestion-causes-error, I-false-positive, L-nursery
fn main() {}

fn example(input: Option<usize>) -> String {
    let string: &dyn std::fmt::Display = if let Some(input) = input {
        &format!("input: {input}")
    } else {
        &"{none}"
    };

    format!("Example: {string}")
}
