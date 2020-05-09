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
    let a: i64 = read();
    let b: i64 = read();
    let mut v: Vec<i64> = vec![];

    for _ in 0..n {
        let hi = read();
        v.push(hi)
    }

    let l = -1;
    let r = 1_000_000_005;
    let mut ri = r;
    let mut li = l;
    let mut cnt = 0;
    while li + 1 < ri {
        let c = (ri + li) / 2;
        cnt = 0;
        for i in 0..v.len() {
            let vi = v[i];
            if vi > b * c {
                cnt += (vi - b * c) / (a - b) + if (vi - b * c) % (a - b) != 0 { 1 } else { 0 };
            }
        }
        if cnt <= c {
            ri = c;
        } else {
            li = c;
        }
    }
    println!("{}", ri);
}
