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
    let k: i64 = read();
    let mut a: Vec<usize> = read_n(n);
    let mut f: Vec<usize> = read_n(n);
    a.sort();
    f.sort();
    f.reverse();

    let mut maxv: i64 = 1_000_000_000_000;
    let mut minv: i64 = -1;
    while minv + 1 < maxv {
        let mid = (maxv + minv) / 2;
        // println!("mid{}, maxv: {}, minv: {}", mid, maxv, minv);
        let mut s: i64 = 0;
        for i in 0..n {
            s += max((a[i] as i64 - (mid / f[i] as i64) as i64), 0);
        }
        if s <= k {
            maxv = mid;
        } else {
            minv = mid;
        }
    }

    println!("{}", maxv);
}
