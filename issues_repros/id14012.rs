//ISSUE #14012 - C-bug, I-false-positive

struct WrapErr(String);

impl From<&str> for WrapErr {
    fn from(val: &str) -> Self {
        todo!()
    }
}

fn foo() -> Result<(), WrapErr> {
    None.ok_or("".into())?;
    Ok(())
}
