//ISSUE #15035 - C-bug, I-suggestion-causes-error, I-false-positive

trait Convert<T> {
    fn from(value: T) -> Self;
}

impl Convert<u64> for () {
    fn from(_value: u64) -> Self {}
}

fn handle<T>(value: u64) -> T where T: Convert<u64> {
    eprintln!("error: {value}");
    Convert::from(value)
}

pub fn f() -> Option<bool> {
    let result: Result<bool, u64> = Err(42);
    result.map_err(|err| -> () { handle(err) }).ok()
}

pub fn main() {
    dbg!(f());
}
