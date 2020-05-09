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
    let k: usize = read();
    let s: usize = read();
    let mut cnt = 0;
    let mut memo: Vec<i64> = vec![0; 2500 * 2 + 2];

    for i in 0..k + 1 {
        for j in 0..k + 1 {
            memo[i + j] += 1;
        }
    }

    for i in 0..k + 1 {
        if s >= i && s - i <= 5000 {
            cnt += memo[s - i];
        }
    }
    println!("{}", cnt);
}
