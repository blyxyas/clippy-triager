//ISSUE #13814 - C-bug, I-false-positive

#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

const X1: u16 = 1251_u16.ilog(3) as u16;
const X2: u16 = 1251_u16.ilog(3) as _;

fn main() {}
