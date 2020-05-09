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

use std::cmp::min;

fn main() {
    let n: usize = read();
    let t: i64 = read();
    let mut a: Vec<i64> = vec![];
    for _ in 0..n {
        let ai = read();
        a.push(ai)
    }

    let mut ans = t * n as i64;

    for i in 1..n {
        let ai_prev = a[i - 1];
        let ai = a[i];
        if ai_prev + t > ai {
            ans -= (ai_prev + t) - ai;
        }
    }
    println!("{}", ans);
}
