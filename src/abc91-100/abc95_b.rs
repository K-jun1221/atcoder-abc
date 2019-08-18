
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
  let x = read::<i64>();
  let mut min_d = 10000000;
  let mut cost = 0;

  for _ in 0..n {
    let m_i = read::<i64>();
    min_d = min(min_d, m_i);
    cost += m_i
  }

  println!("{}", n + ((x - cost) / min_d));
}

