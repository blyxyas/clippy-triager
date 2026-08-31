//ISSUE #17660 <https://github.com/rust-lang/rust-clippy/issues/17660> - C-bug, I-false-negative

    // T: Ord is implied by the requirements of BTreeMap
    fn btreemap_normal<T: ElementExists + Ord, U: ElementExists>() {
        let mut btreemap = BTreeMap::new();
        drop(btreemap.insert(T::ELEMENT, U::ELEMENT));
        assert!(btreemap.is_empty());
    }

    fn btreemap_eq<
        T: ElementExists + Ord + Debug, // T now needs Debug
        U: ElementExists + PartialEq + Debug, // U now needs PartialEq and Debug
    >() {
        let mut btreemap = BTreeMap::new();
        drop(btreemap.insert(T::ELEMENT, U::ELEMENT));
        assert_eq!(btreemap, BTreeMap::new());
    }

fn main() {}