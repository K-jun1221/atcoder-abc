
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

fn gcd(a: i64, b: i64) -> i64 {
  if a % b == 0 {
    b
  } else {
    gcd(b, a % b)
  }
}

fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}

fn main() {
  let mut a = read::<i64>();
  let mut b = read::<i64>();
  let mut c = read::<i64>();
  let mut d = read::<i64>();
  let mut cnt = 0;

  let min_mul = lcm(c, d);

  let m = (b / c) - ((a - 1) / c);
  let m2 = (b / d) - ((a - 1) / d);
  let m3 = (b / min_mul) - ((a - 1) / min_mul);
  println!("{}", (b - a + 1) - (m + m2 - m3));
}
