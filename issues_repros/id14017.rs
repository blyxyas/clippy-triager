//ISSUE #14017 - C-bug, I-false-positive

pub fn print_1_arg() {
    #![expect(clippy::disallowed_macros)]
    print!("foo");
}
pub fn print_2_args() {
    #![expect(clippy::disallowed_macros)]
    print!("{}", 123);
}
pub fn print_3_args() {
    #![expect(clippy::disallowed_macros)]
    print!("{} {}", 123, 456);
}

pub fn panic_1_arg() {
    #![expect(clippy::disallowed_macros)]
    panic!("foo");
}
pub fn panic_2_args() {
    #![expect(clippy::disallowed_macros)]
    panic!("{}", 123);
}
pub fn panic_3_args() {
    #![expect(clippy::disallowed_macros)]
    panic!("{} {}", 123, 456);
}
