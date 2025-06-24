//ISSUE #14009 - C-bug, I-false-negative

let src = [0; 64];
let mut dst = [0; 128];
for (i, &val) in src.iter().enumerate() {
    dst[i + 64] = val;
}
