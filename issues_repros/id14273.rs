//ISSUE #14273 - C-bug

#![warn(clippy::std_instead_of_core)]

use std::{os, ptr};

fn main() {
    let _ = os::raw::c_char::abs;
    let _ = ptr::null::<u8>;
}
