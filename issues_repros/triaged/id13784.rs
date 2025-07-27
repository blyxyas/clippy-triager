//ISSUE #13784 - C-bug, I-false-negative

fn main() {
    let text: String = std::iter::repeat_n('a',5).collect();
    println!("{text}");
}
