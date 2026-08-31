//ISSUE #16235 <https://github.com/rust-lang/rust-clippy/issues/16235> - C-bug, I-false-negative

#![allow(clippy::disallowed_names)]

enum Baz {
    Qux
}

struct Quux {
    #[allow(dead_code)]
    corge: bool,
}

fn main() {
    let foo = Some((2, 4));
    let bar = Some(Baz::Qux);
    let grault = Some(Quux {
        corge: true,
    });

    if let Some((_, _)) = foo {}
    if let Some(Baz::Qux) = bar {}
    if let Some(Quux { corge: _ }) = grault {}
}

