//ISSUE #11754 - C-bug, I-false-positive

// edition:2021
// check-pass
#![feature(coroutines)]

fn main() {
    let x = &mut ();
    || {
        let _c = || yield *&mut *x;
        || _ = &mut *x;
    };
}

