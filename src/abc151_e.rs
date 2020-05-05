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
use std::collections::BTreeMap;

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
            return 0;
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m) % self.m
        }
    }
}

const MOD: i64 = 1_000_000_007;

fn main() {
    let n: usize = read();
    // let mut m: BTreeMap<i64, i64> = BTreeMap::new();
    let k: usize = read();
    let mut an: Vec<i64> = read_n(n);
    an.sort();
    let c = Comb::new(n + 1, MOD);

    let mut ans = 0;
    let mut cnt = n as i64;
    for i in (0..an.len()).rev() {
        let ai = an[i];
        ans += ai * (c.ncr(i, (k - 1) as usize) % MOD);
        ans %= MOD;
        if ans < 0 {
            ans += MOD
        }
    }
    for i in 0..n {
        let ai = an[i];
        ans -= ai * ((c.ncr(an.len() - i - 1, (k - 1) as usize) + MOD) % MOD);
        ans %= MOD;
        if ans < 0 {
            ans += MOD;
        }
    }

    println!("{}", ans);
}
