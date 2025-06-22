//ISSUE #15056 - C-bug

fn main() {
    let b = false;
    std::fs::read("/ENOENT").expect(if b {
        "a"
    } else {
        "b"
    });
}
