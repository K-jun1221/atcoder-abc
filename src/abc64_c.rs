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

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = vec![];
    let mut b: Vec<bool> = vec![false; 8];
    let mut minv: i64 = 0;
    let mut maxv: i64 = 0;
    for _ in 0..n {
        let ai = read();
        a.push(ai);
    }

    for i in 0..a.len() {
        let ai = a[i];
        if ai / 400 < 8 {
            b[(ai / 400) as usize] = true;
        } else {
            maxv += 1;
        }
    }

    for i in 0..b.len() {
        let bi = b[i];
        if bi {
            minv += 1;
            maxv += 1;
        }
    }
    if minv == 0 {
        println!("{} {}", 1, maxv);
    } else {
        println!("{} {}", minv, maxv);
    }
}
