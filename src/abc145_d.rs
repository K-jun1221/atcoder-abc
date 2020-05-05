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
use std::collections::HashMap;

pub struct Comb {
    max: usize,
    m: i64,
    fac: Vec<i64>,
    finv: Vec<i64>,
}
impl Comb {
    /// initialize table
    /// max: maximum that n as nCk takes
    /// m: MOD, a prime number
    /// O(n)
    pub fn new(max: usize, m: i64) -> Comb {
        let n = max;
        let mut fac = Vec::with_capacity(n);
        fac.push(1);
        fac.push(1);
        let mut finv = Vec::with_capacity(n);
        finv.push(1);
        finv.push(1);
        let mut inv = Vec::with_capacity(n);
        inv.push(1);
        inv.push(1);
        for i in 2..n {
            let i = i as i64;
            let tmp = fac.last().unwrap() * i % m;
            fac.push(tmp);
            let tmp = m - inv[(m % i) as usize] * (m / i) % m;
            inv.push(tmp);
            let tmp = finv.last().unwrap() * inv.last().unwrap() % m;
            finv.push(tmp);
        }
        Comb {
            max: max,
            m: m,
            fac: fac,
            finv: finv,
        }
    }
    /// nCr
    pub fn ncr(&self, n: usize, k: usize) -> i64 {
        // since n, k are usize, do not need to check `n < 0 || k < 0`
        if n < k || n > self.max {
            panic!("Invalid query (n, k) = ({}, {})", n, k);
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m) % self.m
        }
    }
}

struct Edge {
    to: i32,
    id: i32,
}
const MOD: i64 = 1_000_000_007;

fn main() {
    let x: i64 = read();
    let y: i64 = read();
    if (x + y) % 3 != 0 {
        println!("{}", 0);
        return;
    }

    let n: i64 = (2 * y - x) / 3;
    if n < 0 {
        println!("{}", 0);
        return;
    }
    let m: i64 = (2 * x - y) / 3;
    if m < 0 {
        println!("{}", 0);
        return;
    }
    let c: Comb = Comb::new((n + m + 1) as usize, MOD);
    let c: Comb = Comb::new((n + m + 1) as usize, MOD);
    let mut ans = c.ncr((n + m) as usize, n as usize);

    println!("{}", ans % MOD);
}
