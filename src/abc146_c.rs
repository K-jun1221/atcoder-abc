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
    let a: usize = read();
    let b: usize = read();
    let x: usize = read();

    let mut ans = 0;
    let mut top = 100_000_000_0;
    if (a * (top) + b * keta(top)) <= x {
        println!("{}", top);
        return;
    }
    let mut done = false;
    let mut bottom = 0;

    while top - bottom != 1 {
        let i = (top + bottom) / 2;
        if (a * i + b * keta(i)) <= x {
            bottom = i;
        } else {
            top = i;
        }
    }
    println!("{}", (top + bottom) / 2);
}

fn keta(a: usize) -> usize {
    let mut x = a.clone();
    let mut num = 0;
    while x != 0 {
        x = (x as i64 / 10) as usize;
        num += 1;
    }
    return num;
}
