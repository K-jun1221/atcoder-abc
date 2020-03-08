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

use std::cmp::{max, min};

fn main() {
    let s = read::<String>();

    let s_char = s.chars().collect::<Vec<char>>();
    let mut ans = 0;

    for i in s_char {
        if i == '+' {
            ans += 1;
        } else {
            ans -= 1;
        }
    }

    println!("{}", ans);
}
