//ISSUE #16229 <https://github.com/rust-lang/rust-clippy/issues/16229> - C-bug

use std::{
    io::{self, Read},
    mem::MaybeUninit,
    slice,
};

fn x<S>(reader: &mut impl Read) -> io::Result<S>
where
    S: Copy,
{
    let mut t = MaybeUninit::uninit();
    let size_of_t = size_of_val(&t);
    let slice_over_t = unsafe { slice::from_raw_parts_mut(t.as_mut_ptr() as *mut u8, size_of_t) };

    reader.read_exact(slice_over_t)?;

    Ok(unsafe { t.assume_init() })
}

fn main() {
    let mut slice: &[u8] = &[0xef, 0xbe, 0xad, 0xde];
    println!("Hello, {:#x}", x::<u32>(&mut slice).unwrap());
}

