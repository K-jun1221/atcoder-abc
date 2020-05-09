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
    let s: String = read();
    let mut alock = false;
    let mut aidx = 0;
    let mut zidx = 0;

    for (i, v) in s.chars().enumerate() {
        if v == 'A' && !alock {
            aidx = i;
            alock = true;
        }
        if v == 'Z' {
            zidx = i
        }
    }
    println!("{}", zidx - aidx + 1);
}
