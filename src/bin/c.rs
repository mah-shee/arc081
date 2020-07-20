#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut pair = vec![0; n];
    a.sort();
    let mut i = n;
    let mut skip = false;
    while i > 1 {
        if skip {
            skip = false;
            i -= 1;
            continue;
        }
        if a[i - 1] == a[i - 2] {
            if pair[0] == 0 {
                pair[0] = a[i - 2];
            } else {
                pair[1] = a[i - 2];
                break;
            }
            skip = true;
        }
        i -= 1;
    }
    println!("{}", pair[0] * pair[1]);
}
