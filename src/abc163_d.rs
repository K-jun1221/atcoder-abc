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
    let n: i64 = read();
    let k: i64 = read();

    let mut a: Vec<i64> = vec![];
    let mut ans: i64 = 0;

    for i in 1..n + 2 {
        a.push(i)
    }

    let mut minv: i64 = a[0..k as usize].iter().sum();
    let mut maxv: i64 = a[a.len() - k as usize..a.len()].iter().sum();
    ans += maxv - minv + 1;

    for i in k + 1..n + 2 {
        minv += a[i as usize - 1];
        maxv += a[a.len() - i as usize];

        ans += maxv - minv + 1;
        if ans > MOD {
            ans %= MOD
        }
    }
    println!("{}", ans);
}
