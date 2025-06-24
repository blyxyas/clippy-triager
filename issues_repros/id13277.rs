//ISSUE #13277 - C-bug, I-false-positive

#![warn(clippy::use_self)]
trait Foo {
    type Item<'foo>;
}
struct Bar<'b> {
    content: &'b str,
}
impl<'b> Foo for Option<Bar<'b>> {
    type Item<'foo> = Option<Bar<'foo>>;
}
