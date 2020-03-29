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
    let h: usize = read();
    let n: usize = read();

    let mut wz: Vec<(usize, i64)> = vec![];

    for _ in 0..n {
        let ai = read();
        let bi = read();

        wz.push((ai, bi))
    }

    wz.sort_by(|&a, b| a.0.cmp(&b.0));
    // wz.reverse();
    let mut dp: Vec<Vec<i64>> = vec![];
    for _ in 0..n + 1 {
        dp.push(vec![std::i64::MAX; h as usize + 1])
    }

    for i in 0..n + 1 {
        dp[i][0] = 0;
    }

    for i in 1..n + 1 {
        for j in 0..h + 1 {
            let is = i as usize;
            let js = j as usize;

            if dp[is][js] == std::i64::MAX && dp[is - 1][js] != std::i64::MAX {
                dp[is][js] = dp[is - 1][js];
            }
            if dp[is][js] == std::i64::MAX {
                continue;
            }

            let mp = wz[is - 1].1;
            let e = wz[is - 1].0;
            let jse = if js + e > h { h } else { js + e };

            dp[is][jse] = min(dp[is - 1][jse], min(dp[is][js] + mp, dp[is][jse]))
        }
    }
    println!("{}", dp[n][h]);
    // for i in dp {
    //     println!("{:?}", i);
    // }
}
