
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

fn main() {
    let w: i64 = read();
    let a: i64 = read();
    let b: i64 = read();
    if a < b {
        if a + w < b {
            println!("{}", b - (w + a));
        } else {
            println!("{}", 0);
        }
    } else {
        if b + w < a {
            println!("{}", a - (b + w));
        } else {
            println!("{}", 0);
        }
    }
}

