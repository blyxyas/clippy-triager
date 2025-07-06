//ISSUE #14322 - C-bug, I-suggestion-causes-error, I-false-positive
fn main() {}

pub trait MyTrait {
    type Borrowed<'a>;
}

pub fn make_static<T: MyTrait>(x: T::Borrowed<'_>) -> T::Borrowed<'static>
where
    for<'a> T::Borrowed<'a>: Into<T::Borrowed<'static>>,
{
    x.into()
}
