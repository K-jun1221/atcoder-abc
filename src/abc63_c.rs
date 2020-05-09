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

use std::collections::HashMap;

fn main() {
    let n: usize = read();
    let mut a: Vec<usize> = vec![];
    let mut sum: usize = 0;

    for _ in 0..n {
        let ai: usize = read();
        if ai % 10 != 0 {
            a.push(ai);
        }
        sum += ai;
    }
    a.sort();

    if sum % 10 == 0 {
        if a.len() == 0 {
            println!("0");
        } else {
            println!("{}", sum - a[0]);
        }
    } else {
        println!("{}", sum);
    }
}
