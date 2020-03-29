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

fn isKaibun(s: &str) -> bool {
    let mut check = true;
    let mut cnt = 0;
    let s_char: Vec<char> = s.chars().collect();
    let s_len = s.len();
    while cnt < s_len / 2 {
        if s_char[cnt] != s_char[s_len - cnt - 1] {
            check = false
        }

        cnt += 1;
    }
    check
}

fn main() {
    let s: String = read();
    let s_len = s.len();
    let a = &s[0..(s_len - 1) / 2];
    let b = &s[(s_len + 3) / 2 - 1..];
    if isKaibun(&s) && isKaibun(a) && isKaibun(b) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
