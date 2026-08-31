//ISSUE #16961 <https://github.com/rust-lang/rust-clippy/issues/16961> - C-bug, I-false-negative

let file_name = OsString::from("XYz");
let ret = file_name.to_str().is_some_and(|dir| {
    dir.chars()
        .map(|c| c.to_ascii_lowercase())
        .cmp(".xyz".chars().map(|c| c.to_ascii_lowercase()))
        .is_eq()
});

fn main() {}