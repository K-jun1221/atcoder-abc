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
    let a: Vec<char> = read::<String>().chars().collect();
    let b: Vec<char> = read::<String>().chars().collect();
    let c: Vec<char> = read::<String>().chars().collect();

    if a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0] {
        println!("YES")
    } else {
        println!("NO")
    }
}
