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
    let n: usize = read();
    let mut a: Vec<i64> = vec![];
    for _ in 0..n {
        let ai: i64 = read();
        a.push(ai);
    }

    let mut ans = 0;
    let mut ap = vec![0; 1_001_000_000];

    for i in 0..a.len() {
        let ai = a[i] as usize;
        ap[i + ai] += 1;
        if i as i64 - ai as i64 >= 0 {
            ans += ap[i - ai];
        }
    }

    println!("{}", ans);
}
