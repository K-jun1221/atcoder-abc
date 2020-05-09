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

use std::cmp::min;

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut students: Vec<(i64, i64)> = vec![];
    let mut checkmarks: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        let a: i64 = read();
        let b: i64 = read();
        students.push((a, b));
    }
    for _ in 0..m {
        let c: i64 = read();
        let d: i64 = read();
        checkmarks.push((c, d));
    }

    for s in &students {
        let mut minv: i64 = std::i64::MAX;
        let mut minpoint = 0;
        for i in 0..checkmarks.len() {
            let ci = checkmarks[i];
            let dist = (ci.0 - s.0).abs() + (ci.1 - s.1).abs();
            if dist < minv {
                minv = dist;
                minpoint = i;
            }
        }
        println!("{}", minpoint +1);
    }
}
