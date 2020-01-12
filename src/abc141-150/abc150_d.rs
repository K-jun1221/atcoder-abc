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

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i64, y: i64) -> i64 {
    x * (y / gcd(x, y))
}

use std::cmp::{max, min};
fn main() {
    let n: usize = read();
    let m: i64 = read();
    let a: Vec<i64> = read_n(n);
    let b: Vec<i64> = a.iter().map(|x| x / 2).collect();
    if b.len() == 1 {
        if m / b[0] != 0 {
            println!("{}", m / b[0] / 2 + m / b[0] % 2);
        } else {
            println!("{}", 0);
        }
        return;
    }
    let mut c = lcm(b[0], b[1]);

    for i in b {
        c = lcm(c, i)
    }

    if m / c != 0 {
        println!("{}", m / c / 2 + m / c % 2);
    } else {
        println!("{}", 0);
    }
}
