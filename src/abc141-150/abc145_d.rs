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
const MOD: usize = 1000000007;
fn nCr(n: usize, k: usize) -> usize {
    if n == 0 && k == 0 {
        return 1;
    }
    if n < k {
        return 0;
    }
    let mut fac = vec![1usize; n + 1];
    let mut ifac = vec![1usize; n + 1];
    for i in 0..n {
        fac[i + 1] = fac[i] * (i + 1) % MOD;
        ifac[i + 1] = ifac[i] * mod_power(i + 1, MOD - 2) % MOD;
    }
    let tmp = ifac[n - k] * ifac[k] % MOD;
    tmp * fac[n] % MOD
}
fn mod_power(x: usize, n: usize) -> usize {
    let mut ret = 1;
    let mut xx = x;
    let mut nn = n;
    while nn > 0 {
        if nn & 1 > 0 {
            ret = ret * xx % MOD;
        }
        xx = xx * xx % MOD;
        nn = nn >> 1;
    }
    ret
}

use std::cmp::{max, min};

fn main() {
    let x: i64 = read();
    let y: i64 = read();
    if (x + y) % 3 != 0 {
        println!("{}", 0);
        return;
    }

    let Y = (2 * y - x) / 3;
    let X = (2 * x - y) / 3;
    if 0 < X && 0 < Y {
        println!("{}", nCr(X as usize + Y as usize, X as usize));
    } else {
        println!("{}", 0);
    }
}
