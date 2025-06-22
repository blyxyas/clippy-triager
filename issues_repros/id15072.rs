//ISSUE #15072 - C-bug, I-false-positive

use std::ops::Deref;

fn main() {
    #[allow(non_camel_case_types)]
    struct foo;

    impl Deref for foo {
        type Target = fn() -> &'static str;

        fn deref(&self) -> &Self::Target {
            fn hello() -> &'static str {
                "Hello, world!"
            }
            &(hello as fn() -> &'static str)
        }
    }

    fn accepts_fn(f: impl Fn() -> &'static str) {
        println!("{}", f());
    }

    let f = foo;
    accepts_fn(|| f());
}
