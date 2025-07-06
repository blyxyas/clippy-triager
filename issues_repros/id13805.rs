//ISSUE #13805 - C-bug, I-false-negative
fn main() {}

pub fn linted1(input: Option<i32>) {
    if input.is_some() {
        std::hint::black_box(input.unwrap());
    }
}
pub fn linted2(input: Option<i32>) {
    if input.is_none() {
    } else {
        std::hint::black_box(input.unwrap());
    }
}

pub fn silent1(input: Option<i32>) {
    if input.is_some() {
    } else {
        return;
    }
    std::hint::black_box(input.unwrap());
}
pub fn silent2(input: Option<i32>) {
    if input.is_none() {
        return;
    }
    std::hint::black_box(input.unwrap());
}

