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
    let sx: i64 = read();
    let sy: i64 = read();
    let tx: i64 = read();
    let ty: i64 = read();

    let mut ans: String = String::new();
    let xdist = tx - sx;
    let ydist = ty - sy;

    for _ in 0..xdist {
        ans += "R"
    }

    for _ in sy..ty {
        ans += "U"
    }

    for _ in sx..tx {
        ans += "L"
    }
    for _ in sy..ty {
        ans += "D"
    }
    ans += "D";

    for _ in sx..tx + 1 {
        ans += "R"
    }

    for _ in 0..ydist + 1 {
        ans += "U"
    }
    ans += "LU";
    for _ in 0..xdist + 1 {
        ans += "L"
    }

    for _ in 0..ydist + 1 {
        ans += "D"
    }
    ans += "R";
    println!("{}", ans);
}
