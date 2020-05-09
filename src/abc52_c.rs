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

fn is_prime(a: i64) -> bool {
    if a == 1 {
        return false;
    }
    for i in 2..a {
        if a % i == 0 {
            return false;
        }
    }
    return true;
}

const MOD: i64 = 1_000_000_007;

fn main() {
    let n: i64 = read();
    let mut prime: Vec<i64> = vec![];
    for i in 1..n + 1 {
        if is_prime(i) {
            prime.push(i);
        }
    }
    // println!("{:?}", prime);
    let mut a: Vec<i64> = vec![0; prime.len()];
    for i in 0..prime.len() {
        let pi = prime[i];
        let mut current = pi;
        while current <= n {
            a[i] += n / current;
            current *= pi;
        }
    }
    let mut sum = 1;
    for i in a {
        sum *= i + 1;
        sum %= MOD;
    }
    println!("{}", sum);
}
