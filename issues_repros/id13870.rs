//ISSUE #13870 - C-bug, I-false-positive

let mut collection = [4, 3, 2, 1];
    let mut i = 0;
    'outer: while i < collection.len() {
        for j in 0..i {
            if collection[j] > collection[i] {
                collection.swap(j, i);
                i -= 1;
                continue 'outer;
            }
        }

        i += 1;
    }
