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
    let n: i64 = read();
    let s: String = read();
    let mut maxv = 0;
    let mut cv = 0;

    for i in s.chars() {
        if i == 'I' {
            cv += 1;
        } else {
            cv -= 1;
        }
        maxv = max(maxv, cv);
    }

    println!("{}", maxv);
}
