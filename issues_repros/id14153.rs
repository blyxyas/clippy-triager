//ISSUE #14153 - C-bug, T-macros, I-suggestion-causes-error, I-false-positive
fn main() {}

pub fn check() -> bool {
    std::is_x86_feature_detected!("sse4.2") || std::is_x86_feature_detected!("avx2")
}
