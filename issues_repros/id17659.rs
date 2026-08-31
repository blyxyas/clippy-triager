//ISSUE #17659 <https://github.com/rust-lang/rust-clippy/issues/17659> - C-bug, I-false-negative

#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct HasDebug;

    #[test]
    fn empty_vec_is_empty() {
        let empty: Vec<HasDebug> = Vec::new();
        assert!(empty.is_empty());
    }

    #[test]
    fn vec_with_one_is_not_empty() {
        let one: Vec<HasDebug> = vec![HasDebug];
        assert!(!one.is_empty());
    }

    #[test]
    fn vec_with_multiple_is_not_empty() {
        let multiple: Vec<HasDebug> = vec![HasDebug, HasDebug];
        assert!(!multiple.is_empty());
    }

    #[test]
    #[should_panic]
    fn empty_vec_is_not_empty() {
        let empty: Vec<HasDebug> = Vec::new();
        assert!(!empty.is_empty());
    }

    #[test]
    #[should_panic]
    fn vec_with_one_is_empty() {
        let one: Vec<HasDebug> = vec![HasDebug];
        assert!(one.is_empty());
    }

    #[test]
    #[should_panic]
    fn vec_with_multiple_is_empty() {
        let multiple: Vec<HasDebug> = vec![HasDebug, HasDebug];
        assert!(multiple.is_empty());
    }
}

fn main() {}