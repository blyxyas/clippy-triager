//ISSUE #16382 <https://github.com/rust-lang/rust-clippy/issues/16382> - C-bug, I-false-negative

// src/lib.rs:
#![warn(clippy::empty_docs)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        type R;

        ///
        unsafe fn my_fn(&self);
    }
}

pub fn public_my_fn(r: &ffi::R) {
    unsafe { r.my_fn() }
}

fn main() {}