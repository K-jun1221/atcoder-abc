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
    let a: f64 = read();
    let b: f64 = read();
    let x: f64 = read();

    if (a * a * b) - x < (a * a * b / 2.0) {
        let ans = (2.0 * ((a * a * b) - x)) / (a * a * a);
        let abs_difference = ans.atan().to_degrees();
        println!("{}", abs_difference);
    } else {
        let ans = (a - (a - ((2.0 * x) / (a * b)))) / b;
        let abs_difference = ans.atan().to_degrees();
        println!("{}", 90.0 - abs_difference);
    }
}
