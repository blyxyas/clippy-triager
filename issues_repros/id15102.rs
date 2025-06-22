//ISSUE #15102 - C-bug, I-suggestion-causes-error

fn main() {
    let values = [None, Some(3)];
    let has_even = values.iter().find(|v| matches!(v, Some(x) if x % 2 == 0)).is_some();
    println!("{has_even}");
}
