//ISSUE #17629 <https://github.com/rust-lang/rust-clippy/issues/17629> - C-bug, I-suggestion-causes-error, I-false-positive

#![deny(clippy::filter_map_bool_then)]

pub fn main() {
    let mut last = None;
    let dedup: Vec<u32> = [1, 2, 2, 3]
        .into_iter()
        .filter_map(|val| {
            (last != Some(val)).then(|| {
                last = Some(val);
                val
            })
        })
        .collect();
    println!("{:?}", dedup);
}

