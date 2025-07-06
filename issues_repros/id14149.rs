//ISSUE #14149 - C-bug, I-false-positive
fn main() {}

use std::cmp::Ordering;

struct A {
    inner: i32,
    cache: i32,
}

impl PartialEq for A {
    fn eq(&self, other: &A) -> bool {
        self.inner == other.inner
    }
}

impl Eq for A {}

impl PartialOrd for A {
    fn partial_cmp(&self, other: &A) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for A {
    fn cmp(&self, other: &A) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
