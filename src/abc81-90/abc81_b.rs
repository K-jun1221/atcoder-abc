
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

fn main() {
  let n = read::<i64>();
  let mut ans = 100000;

  for _ in 0..n {
    let mut cnt = 0;
    let mut n = read::<i64>();
    while n % 2 == 0 {
      n = n / 2;
      cnt += 1;
    }
    ans = min(ans, cnt)
  }

  println!("{}", ans);
}

