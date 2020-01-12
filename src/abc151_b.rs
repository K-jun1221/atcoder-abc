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
    let m: usize = read();
    let mut qs: Vec<u8> = vec![0; n];
    let mut ps: Vec<i64> = vec![0; n];

    for _ in 0..m {
        let qn: usize = read();
        let status: String = read();
        if qs[qn - 1] != 0 {
            continue;
        }
        if status == "AC" {
            qs[qn - 1] = 1;
        }
        if status == "WA" {
            ps[qn - 1] += 1;
        }
    }

    let mut ans = 0;
    for i in qs {
        ans += i;
    }

    let mut ans2 = 0;
    for i in ps {
        ans2 += i;
    }

    println!("{} {}", ans, ans2);
}
