//ISSUE #14215 - C-bug, I-suggestion-causes-error, I-false-positive

use core::marker::PhantomData;
use core::ops::FnOnce;

struct Guard<'a, S> {
    _data: PhantomData<&'a S>,
}

impl<S> Guard<'_, S> {
    fn do_something(_this: &mut Self) {}
}

fn takes_guard_fn<S, F>(_func: F)
where
    F: FnOnce(&mut Guard<S>),
{
}

fn main() {
    takes_guard_fn::<i32, _>(|s| Guard::do_something(s));
}
