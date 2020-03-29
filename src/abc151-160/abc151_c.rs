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
    let mut n: usize = read();
    let mut m = read();

    let mut oks = vec![false; n + 1];
    let mut fails = vec![0; n + 1];

    for _ in 0..m {
        let p: usize = read();
        let s: String = read();
        if oks[p] == false && s == "WA" {
            fails[p] += 1;
        }
        if s == "AC" {
            oks[p] = true;
        }
    }
    let mut cnt =0;

    for (i, &v) in fails.iter().enumerate() {

        if oks[i] {
            cnt += v;
        }
    }

    println!("{} {}", oks.iter().filter(|&x| *x == true).count(), cnt)
}
