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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        }
    }
}
use std::cmp::{max, min};

fn main() {
    let a: i64 = read();
    let b: i64 = read();
    let c: i64 = read();
    let x: i64 = read();
    let y: i64 = read();
    let mut ans = 1_000_000_000;

    let ab = 2 * c;

    for i in 0..max(x, y)+1 {
        let mut maybe_ans = i * ab;
        
        if (i < x) {
            maybe_ans += a * (x-i);
        } 
        if (i < y) {
            maybe_ans += b * (y-i);
        }
        ans = min(ans, maybe_ans);
    }
    println!("{}", ans);
}
