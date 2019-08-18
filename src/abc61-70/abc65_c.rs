
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

use std::cmp::{max, min};
use std::collections::HashSet;


fn main() {
  let n = read::<i64>();
  let m = read::<i64>();
  const MOD: i64 = 1000000007;
  let mut ans: i64 = 1;

  if 1 < (n - m).abs() {
    ans = 0;
  } else {
    for i in 1..n + 1 {
      ans *= i;
      ans %= MOD;
    }
    for i in 1..m + 1 {
      ans *= i;
      ans %= MOD;
    }

    if n == m {
      ans *= 2;
      ans %= MOD;
    }
  }

  println!("{}", ans);
}
