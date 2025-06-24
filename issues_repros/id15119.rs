//ISSUE #15119 <https://github.com/rust-lang/rust-clippy/issues/15119> - C-bug

fn foo() {
    let mut current = vec![];
    while !current.is_empty() {
        current.drain(..).for_each(|_: ()| {});
    }
}
