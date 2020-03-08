
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
    let n: i64 = read();
    let m: usize = read();
    let mut map: Vec<Vec<i64>> = vec![];
    let mut ans = 0;
    for _ in 0..n {
        let a = read_n(m);
        map.push(a);
    }

    for i in 0..m {
        for j in 0..m {
            if i == j {
                continue;
            }
            let mut val = 0;
            for k in 0..n {
                val += max(map[k as usize][i], map[k as usize][j]);
            }
            ans = max(ans, val);

        }
    }

    println!("{}", ans);
}
