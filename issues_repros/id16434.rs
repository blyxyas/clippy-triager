//ISSUE #16434 <https://github.com/rust-lang/rust-clippy/issues/16434> - C-bug, I-false-positive

fn main() {
    assert!(std::arch::is_x86_feature_detected!("sse4.2"));
}

