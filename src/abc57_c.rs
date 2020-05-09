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
    let n: i64 = read();
    let mut d = 1;
    let mut minv = std::i64::MAX;
    while d * d <= n {
        if n % d == 0 {
            let a = d;
            let b = n / d;
            minv = min(minv, max(a.to_string().len(), b.to_string().len()) as i64);
        }
        d += 1;
    }
    println!("{}", minv);
}
