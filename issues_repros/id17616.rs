//ISSUE #17616 <https://github.com/rust-lang/rust-clippy/issues/17616> - C-bug, I-false-negative

#[warn(match_like_matches_macro)]
fn foo(){
    match match_type {
        MatchType::A(_str1) => true, // this bool != next one
        MatchType::B(_str1, _str2) => false,
        MatchType::C => true,
    }
}

fn main() {}