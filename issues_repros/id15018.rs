//ISSUE #15018 - C-bug, I-suggestion-causes-bug

fn main() {
    let a = 10;
    match 11 {
        a => { println!("a = {a}"); }
    }
    println!("a = {a}");
}
