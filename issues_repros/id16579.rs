//ISSUE #16579 <https://github.com/rust-lang/rust-clippy/issues/16579> - C-bug

            match parse_char_escape(line) {
                Some(decoded) => result.push(decoded),
                None => {
                    result.push('\\');
                    result.push(line.current());
                    line.advance();
                }
            }

fn main() {}