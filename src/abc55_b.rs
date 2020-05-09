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

const MOD: i64 = 1_000_000_007;

fn main() {
    let mut n: usize = read();
    let mut ans: i64 = 1;
    let mut cnt: i64 = 1;

    while n > 0 {
        ans *= cnt;
        ans %= MOD;
        cnt += 1;
        n -= 1;
    }
    println!("{}", ans);
}
