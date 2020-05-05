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
use std::collections::HashSet;

fn main() {
    let s: String = read();
    let sc: Vec<char> = s.chars().collect();
    let a: Vec<Vec<i64>> = vec![vec![0; s.len() as usize]; s.len() as usize];
    let mut sum: i64 = 0;

    for i in 2..s.len() {
        let sci = sc[i].to_digit(10).unwrap();
        sum += sci as i64;
        sum %= 3;
        a[1][i] == sum;
    }
}
