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
    let n: i64 = read();
    let mut p = vec![];
    for i in 0..n {
        let a: i64 = read();
        let b: i64 = read();
        p.push((a, b))
    }
    let mut ans = 0.0;

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let a = p[i as usize].0;
                let b = p[i as usize].1;
                let c = p[j as usize].0;
                let d = p[j as usize].1;
                ans += ((a - c).pow(2) as f64 + (b - d).pow(2) as f64).sqrt();
            }
        }
    }
    println!("{}", ans / n as f64);
}
