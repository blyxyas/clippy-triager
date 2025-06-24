//ISSUE #13356 - C-bug, I-false-positive

#[expect(clippy::derived_hash_with_manual_eq)]
#[derive(Hash)]
pub struct Thing;

impl PartialEq<Thing> for Thing {
    fn eq(&self, _other: &Thing) -> bool {
        true
    }
}
