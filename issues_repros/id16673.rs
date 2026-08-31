//ISSUE #16673 <https://github.com/rust-lang/rust-clippy/issues/16673> - C-bug, I-false-positive

fn main() {
    let vec = vec();

    dbg!(vec[1]);
}

fn vec() -> Vec<i32> {
    vec![0, 0, 0]
}


