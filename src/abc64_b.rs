use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

use std::cmp::max;
use std::cmp::min;

fn main() {
    let n: usize = read();
    let mut maxv = 0;
    let mut minv = std::i64::MAX;
    for _ in 0..n {
        let a: i64 = read();
        maxv = max(maxv, a);
        minv = min(minv, a);
    }
    println!("{}", maxv- minv);
}
