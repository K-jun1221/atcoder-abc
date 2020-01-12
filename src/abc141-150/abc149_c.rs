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
use std::collections::HashSet;

fn divisors(N: i64) -> HashSet<i64> {
    let mut ds = vec![];
    let mut d = 1;
    while d * d <= N {
        if N % d == 0 {
            ds.push(d);
            ds.push(N / d);
        }
        d += 1;
    }
    let uniq: HashSet<i64> = ds.into_iter().collect();
    uniq
}

fn main() {
    let n: i64 = read();
    for i in n..1000000 {
        if divisors(i).len() == 2 {
            println!("{}", i);
            break;
        }
    }
}
