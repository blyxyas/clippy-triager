//ISSUE #13892 <https://github.com/rust-lang/rust-clippy/issues/13892> - C-bug, I-suggestion-causes-error, I-false-positive

pub async fn visit_async<'a, T, const N: usize, F>(array: &'a [T; N], visitor: F)
where
    F: AsyncFn(&'a T),
    T: 'a
{
    //let futures = array.each_ref().map(visitor); // This is what clippy wants me to do, which, i agree, looks nicer, does not actually compile.

    #[allow(unused)] // Temporarily because of todo!()
    let futures = array.each_ref().map(|x| visitor(x));

    // This is where i'd somehow join all the futures in the array `futures` and await them.
    // futures.join_all().await // For example, if join_all was a thing
    todo!()
}
