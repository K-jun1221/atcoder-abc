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
use std::cmp::Ordering;

fn main() {
    let n: usize = read();
    let mut k: usize = read();
    let mut vec: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let a: i64 = read();
        let b: i64 = read();
        vec.push((a, b))
    }
    vec.sort_by(|a, b| {
        if a.0 > b.0 {
            return Ordering::Greater;
        } else {
            if a.0 == b.0 {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }
    });

    for i in 0..n {
        let vi = vec[i];
        if k <= vi.1 as usize {
            println!("{}", vi.0);
            return;
        }
        k -= vi.1 as usize;
    }
}
