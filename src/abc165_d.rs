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
pub struct Comb {
    max: usize,
    m: i64,
    fac: Vec<i64>,
    finv: Vec<i64>,
}
const MOD: i64 = 1_000_000_000;

fn main() {
    let a: f64 = read();
    let b: f64 = read();
    let n: usize = read();
    let mut ans = 0;

    let ans = ((a * min(b as i64 - 1, n as i64) as f64) / b) as i64
        - (a as i64 * (min(b as i64 - 1, n as i64) as f64 / b) as i64) as i64;
    println!("{}", ans);
}
