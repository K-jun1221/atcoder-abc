
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
    let s: String = read();
    let s_char: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut ans = 0;

    for i in 0..1000 {
       let first_digit = if i < 100 { 0 } else { i / 100 };
       let second_digit = if i < 10 { 0 } else { i % 100 / 10 };
       let third_digit = i % 10;

       let f_index: usize = match s_char.iter().position(|x| x == &first_digit) {
           Some(s) => s,
           None => continue
       };
       let s_index: usize = match s_char[f_index+1..].iter().position(|x| x == &second_digit) {
           Some(s) => s,
           None => continue
       };
       match s_char[f_index+1..][s_index+1..].iter().position(|x| x == &third_digit) {
           Some(s) => ans += 1,
           None => continue
       }
    }


    println!("{}", ans);
}
