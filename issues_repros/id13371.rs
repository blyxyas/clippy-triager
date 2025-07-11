//ISSUE #13371 - C-bug, I-false-positive

struct NotFoundError;

fn bar() -> Result<(), NotFoundError> { todo!() }

fn main() {
    match bar() {
        Ok(()) => {},
        // don't need to do anything if not found
        Err(NotFoundError) => {},
    }
}
