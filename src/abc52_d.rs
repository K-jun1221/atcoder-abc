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

fn main() {
    let n: usize = read();
    let a: i64 = read();
    let b: i64 = read();
    let mut x: Vec<i64> = vec![];
    for _ in 0..n {
        let xi = read();
        x.push(xi);
    }
    let mut ans = 0;
    
    for i in 1..n {
        let dist = x[i] - x[i-1];
        if a * dist < b {
            ans += a * dist;
        } else {
            ans += b;
        }
    }
    println!("{}", ans);
}
