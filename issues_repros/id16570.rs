//ISSUE #16570 <https://github.com/rust-lang/rust-clippy/issues/16570> - C-bug, I-false-negative

let opt = Some(Some(1));

let _ = if true && let Some(inner) = opt {
    match inner {
        Some(_) => 0,
        _ => 1,
    }
} else {
    1
};

fn main() {}