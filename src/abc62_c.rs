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

use std::cmp::max;
use std::cmp::min;

fn main() {
    let h: i64 = read();
    let w: i64 = read();

    if h % 3 == 0 || h % 3 == 0 {
        println!("0");
        return;
    }

    let mut minv = std::i64::MAX;
    for i in 1..h {
        let a = (i * w);
        let b = ((h - i) * (w / 2));
        let c = ((h - i) * (w / 2) + (w % 2));
        let s = max(a, max(b, c)) - min(a, min(b, c));
        minv = min(minv, s);
        let d = ((h - i) / 2 + (h - i) % 2) * w;
        let e = ((h - i) / 2) * w;
        let s = max(a, max(d, e)) - min(a, min(d, e));
        minv = min(minv, s);
    }
    for i in 1..w {
        let a = (i * h);
        let b = (w - i) * (h / 2);
        let c = (w - i) * ((h / 2) + (h % 2));
        let s = max(a, max(b, c)) - min(a, min(b, c));
        minv = min(minv, s);
        let d = ((w - i) / 2 + (w - i) % 2) * h;
        let e = ((w - i) / 2) * h;
        let s = max(a, max(d, e)) - min(a, min(d, e));
        minv = min(minv, s);
    }
    println!("{}", minv);
}
