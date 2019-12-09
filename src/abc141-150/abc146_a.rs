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
    let n: usize = read();
    let mut k: i64 = read();
    let mut a: Vec<i64> = read_n(n);
    let mut f: Vec<i64> = read_n(n);
    let ans = 0;
    a.sort();
    f.sort();
    f.reverse();
    println!("{:?}", a);
    println!("{:?}", f);

    for (i, v) in f.iter().enumerate() {
        if 0 < k {
            k -= a[i];
        }

        println!("k: {}", k);
    }
}
