//ISSUE #14004 <https://github.com/rust-lang/rust-clippy/issues/14004> - C-bug, I-suggestion-causes-error

#![deny(clippy::iter_without_into_iter)]
#![allow(unused)]

pub mod inner {
    pub struct S;
    impl S {
        pub fn iter(&self) -> I {
            todo!()
        }
    }

    pub struct I<'a> {
        field: &'a ()
    }
    impl Iterator for I<'_> {
        type Item = ();
        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
}

fn main() {}
