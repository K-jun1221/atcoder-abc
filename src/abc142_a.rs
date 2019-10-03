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
    let mut cnt: f64 = 0.0;

    for i in 1..n + 1 {
        if i % 2 == 1 {
            cnt += 1.0;
        }
    }

    // println!("{}", n);
    // println!("{}", cnt);

    println!("{}", cnt / n as f64);

}
