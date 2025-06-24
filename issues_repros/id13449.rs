//ISSUE #13449 - C-bug

use std::fmt::Write;
fn main() {
    let mut s = String::new();
    write!(s, "{}", "{}").unwrap();
}
