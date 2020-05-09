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

fn main() {
    let n: usize = read();
    let w: usize = read();
    let mut a: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let b = read();
        let c = read();
        a.push((b, c));
    }
    a.sort_by_cached_key(|x| x.0);

    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; n + 1];

    for i in 1..n + 1 {
        let ai = a[i - 1];
        for j in 1..w + 1 {
            if j >= ai.0 as usize {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - ai.0 as usize] + ai.1)
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    println!("{}", dp[n][w]);
}
