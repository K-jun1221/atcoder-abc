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
    let a: usize = read();
    let b: String = read();
    let b_char: Vec<char> = b.chars().collect();
    let ascii_uppercase: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let ans = b_char
        .iter()
        .map(|&c| {
            ascii_uppercase[((0..26).find(|&i| ascii_uppercase[i] == c).unwrap() + a) % 26]
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");
    println!("{}", ans);
}
