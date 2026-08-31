//ISSUE #16317 <https://github.com/rust-lang/rust-clippy/issues/16317> - C-bug, I-false-positive

// src/lib.rs:
#![deny(clippy::missing_safety_doc)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        type R;

        /// # Safety
        ///
        /// Caller needs to ensure that X, Y, Z.
        unsafe fn my_fn(&self);
    }
}

pub fn public_my_fn(r: &ffi::R) {
    // SAFETY: X, Y, and Z are guaranteed based on ...
    unsafe { r.my_fn() }
}

fn main() {}