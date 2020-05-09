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
    let n: usize = read();
    let m: usize = read();
    let mut map: Vec<Vec<i64>> = vec![vec![]; n + 2];
    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        map[a].push(b as i64);
        map[b].push(a as i64);
    }
    for i in 0..n {
        println!("{}", map[i+1].len())
    }
}
