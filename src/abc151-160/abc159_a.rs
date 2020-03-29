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

fn nCr(mut k: i64, n: i64) -> i64 {
    if k > n {
        return 0;
    }
    if k * 2 > n {
        k = n - k
    }
    let mut res = 1;
    for i in 1..k + 1 {
        res *= n - k + i;
        res /= i;
    }
    res
}

fn main() {
    let n: i64 = read();
    let m: i64 = read();
    let nn = nCr(2, n);
    let mm = nCr(2, m);
    println!("{}", nn + mm);
}
