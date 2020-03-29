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
use std::cmp::Ordering;
use std::cmp::{max, min};

fn main() {
    let n = read();
    let k = read();
    let p: Vec<u32> = read_n(n);

    let mut tmp = 0;
    for i in 0..k {
        tmp += p[i];
    }

    let mut sum = tmp;

    for i in k..n {
        tmp = tmp + p[i] - p[i - k];
        if tmp > sum {
            sum = tmp;
        }
    }

    let ans = ((sum + k as u32) as f64 ) / 2.0;
    println!("{}", ans);
}
