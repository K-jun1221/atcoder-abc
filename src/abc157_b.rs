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
    let mut bingo: Vec<i64> = vec![];
    for _ in 0..9 {
        let a: i64 = read();
        bingo.push(a);
    }

    let mut check: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
    let n: i64 = read();
    for _ in 0..n {
        let c: i64 = read();
        match bingo.iter().position(|&x| x == c) {
            Some(s) => {
                check[s / 3][s % 3] = true;
            }
            None => {}
        }
    }
    let mut ans = "No";

    // vertical
    for i in 0..3 {
        if check[i][0] && check[i][1] && check[i][2] {
            ans = "Yes"
        }
    }

    // horizontal
    for i in 0..3 {
        if check[1][i] && check [0][i] && check[2][i] {
            ans = "Yes"
        }
    }

    if check[1][1] && check[0][0] && check[2][2] {
        ans = "Yes"
    }

    if check[0][2] && check[1][1] && check[2][0] {
        ans = "Yes";
    }
    println!("{}", ans);
    
}
