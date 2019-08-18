
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
use std::collections::HashMap;

fn nCr(n: i64, r: i64) -> i64 {
  let mut ans = 1;
  for i in n - r + 1..n + 1 {
    ans *= i;
  }

  for i in 1..r + 1 {
    ans /= i;
  }
  return ans;
}

fn main() {
  let n = read::<i64>();
  let k = read::<i64>();

  for i in 1..k + 1 {
    let a = nCr(n - k + 1, i);
    let b = nCr(k - 1, i - 1);
    println!("{}", (a * b) % 1000000007);
  }
}
