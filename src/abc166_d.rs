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
    let x: i64 = read();
    let mut xv: Vec<bool> = vec![false; x as usize + 1];
    let mut ans = (0, 0);

    for i in 0..120 {
        for j in 0..120 {
            let a = i * i * i * i * i as i64;
            let b = j * j * j * j * j as i64;
            let c = a as i64 * -1;
            let d = b as i64 * -1;
            if a - b == x {
                ans = (i, j);
                break;
            }
            if c - b == x {
                ans = (-i, j);
                break;
            }
            if a - d == x {
                ans = (i, -j);
                break;
            }
            if c - d == x {
                ans = (-i, -j);
                break;
            }
        }
    }
    println!("{} {}", ans.0, ans.1);
}
