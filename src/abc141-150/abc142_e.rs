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
    println!("{}", "oppai");
    println!("this is from vim")

    let n: usize = read();
    let m: usize = read();

    let mut keys: Vec<(i64, i64)> = vec![];

    for _ in 0..m {
        let a: i64 = read();
        let b: usize = read();
        let b_vec: Vec<usize> = read_n(b);
        let mut s = 0;

        for i in b_vec {
            s += 1 << (i - 1)
        }
        keys.push((a, s));
    }

    // println!("n:{}, m: {}", n, m);
    // println!("{:?}", keys);

    let mut dp: Vec<i64> = vec![1000000007; (1 << n) + 5];
    dp[0] = 0;

    for j in 0..(1 << n) {
        // println!("j: {}", j);
        for key in &keys {
            let news = j | key.1;
            dp[news as usize] = min(dp[news as usize], dp[j as usize] + key.0)
        }
    }
    if dp[(1 << n) - 1] == 1000000007 {
        println!("{}", -1);
    } else {
        println!("{}", dp[(1 << n) - 1]);
    }

}
