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

fn calc_score(num_vec: Vec<usize>, qv: &Vec<(usize, usize, usize, usize)>) -> usize {
    let mut score = 0;
    for i in qv {
        if num_vec[i.1 - 1] - num_vec[i.0 - 1] == i.2 {
            score += i.3;
        }
    }
    return score;
}

fn dfs(
    current_num_vec: Vec<usize>,
    n: usize,
    m: usize,
    qv: &Vec<(usize, usize, usize, usize)>,
) -> usize {
    if current_num_vec.len() == n {
        return calc_score(current_num_vec, &qv);
    }
    let mut last_num = 1;
    if current_num_vec.len() != 0 {
        last_num = current_num_vec[current_num_vec.len() - 1];
    }
    let mut maxv = 0;
    for i in last_num..m + 1 {
        let mut num_vec = current_num_vec.clone();
        num_vec.push(i);
        let value = dfs(num_vec, n, m, &qv);
        maxv = max(maxv, value);
    }

    return maxv;
}

const MOD: i64 = 1_000_000_000;
fn main() {
    let n: usize = read();
    let m: usize = read();
    let q: usize = read();
    // let c: Comb = Comb::new(n + m, MOD);
    let mut qv: Vec<(usize, usize, usize, usize)> = vec![];

    for _ in 0..q {
        let a: usize = read();
        let b: usize = read();
        let c: usize = read();
        let d: usize = read();
        qv.push((a, b, c, d));
    }
    let num_vec = vec![];

    let ans = dfs(num_vec, n, m, &qv);
    println!("{}", ans);
}
