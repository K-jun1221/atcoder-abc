
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
    let q: usize = read();
    let a: Vec<i64> = read_n(q);

    let mut point = vec![0; n];

    for i in a {
        let idx = (i - 1) as usize;
        point[idx] += 1;
    }

    for i in point {
        if k - (q as i64 - i) > 0 {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
