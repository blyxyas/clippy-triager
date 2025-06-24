//ISSUE #14095 - C-bug, I-false-positive

pub mod widget {
    #[macro_export]
    macro_rules! define_widget {
        ($id:ident) => { /* ... */ }
    }
}
