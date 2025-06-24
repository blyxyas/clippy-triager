//ISSUE #14238 - C-bug, I-suggestion-causes-error, I-false-positive

struct T<'a> {
    t: Option<&'a str>,
}

impl<'a> T<'a> {
    fn new(t: &'a str) -> Self {
        Self { t: Some(t) }
    }
}

impl Drop for T<'_> {
    fn drop(&mut self) {
    }
}

impl Iterator for T<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.t.take().map(|s| s.to_string())
    }
}

impl DoubleEndedIterator for T<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

fn main() {
    let mut s = String::from("hello");
    let it = T::new(&s);
    println!("last = {}", it.last().unwrap());
    s.push_str(", world!");
    println!("s = {s}");
}

