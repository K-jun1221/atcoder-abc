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
const MOD: i64 = 1_000_000_007;

fn main() {
    let n: usize = read();
    let a: Vec<i64> = read_n(n);
    let mut ans = 0;

    for i in 0..60 {
        let mut cnt = 0;
        for j in 0..n {
            if a[j] >> i & 1 == 1 {
                cnt += 1;
            }
        }
        let a = (cnt * (n - cnt)) as i64 % MOD;
        let b = powmod(2, i) % MOD;
        let c = (a * b) % MOD;
        ans += c;
        if ans < 0 {
            ans += MOD;
        }
        ans %= MOD;
    }
    println!("{}", ans);
}

fn powmod(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        let k = powmod(x, n / 2);
        return (k * k) % MOD;
    } else {
        let k = powmod(x, n / 2);
        return (((k * k) % MOD) * x) % MOD;
    }
}
