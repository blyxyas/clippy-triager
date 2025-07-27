//ISSUE #15067 - C-bug, I-suggestion-causes-error

// src/main.rs
fn main() {
    let x: f64 = 5.0;
    _ = x.round();
}
