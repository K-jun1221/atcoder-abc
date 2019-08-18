
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

fn main() {
  let a = read::<i64>();
  let b = read::<i64>();
  let c = read::<i64>();
  let k = read::<i64>();

  let mut max_v = max(a, max(b, c));
  let mut max_v_mul = max(a, max(b, c));

  for _ in 0..k {
    max_v_mul = max_v_mul * 2;
  }

  println!("{}", (max_v_mul - max_v) + a + b + c);
}

