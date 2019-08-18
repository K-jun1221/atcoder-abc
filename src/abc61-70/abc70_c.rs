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

fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}

fn lcm(x: i64, y: i64) -> i64 {
  x * (y / gcd(x, y))
}

use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
  let a = read::<i64>();
  let mut ans = 1;

  for _ in 0..a {
    let b = read::<i64>();
    ans = lcm(ans, b);
  }

  println!("{}", ans);
}

