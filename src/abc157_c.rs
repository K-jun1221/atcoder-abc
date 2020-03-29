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
    let m: i64 = read();
    let mut a: Vec<i64> = vec![-1; n];
    let mut ans = 0;

    if n == 1 && m == 0 {
        println!("{}", 0);
        return;
    }

    for _ in 0..m {
        let s: usize = read();
        let c: i64 = read();
        if a[s - 1] != -1 && a[s - 1] != c {
            ans = -1;
        }
        a[s - 1] = c
    }

    if ans == -1 || (n != 1 && a[0] == 0) {
        println!("{}", -1);
        return;
    }

    let mut ansa: String = "".to_string();
    for i in 0..n {
        let ai = a[i];
        if ai != -1 {
            ansa += &ai.to_string();
        } else {
            if i != 0 {
                ansa += &"0".to_string();
            } else {
                ansa += &"1".to_string();
            }
        }
    }

    println!("{}", ansa);
}
