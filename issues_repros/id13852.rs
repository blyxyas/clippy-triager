//ISSUE #13852 - C-bug

fn main() {
    let mut a = [vec![1,2,3,4], vec![5,6,7,8]];

    for i in 0..a.len()-1 {
        a[i] = a[i]
            .iter()
            .filter(|&&b| {
                a[i+1]
                    .iter()
                    .any(|&c| c == b)
            })
            .copied()
            .collect();
    }
}
