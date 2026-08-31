//ISSUE #16916 <https://github.com/rust-lang/rust-clippy/issues/16916> - C-bug

use std::env;

#[cfg(feature = "debug")]
#[macro_export]
macro_rules!print_debug {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules!print_debug {
    ($($arg:tt)*) => {};
}


fn main() {
    match env::args().len() {
        2 => {
            println!("Not enough many");
        }
        3 => {
            print_debug!("Let's go");
            if env::args().len() > 5 {
                println!("So many");
            }
        }
        _ => {}
    }
}

