
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
  let mut d = vec![];
  for _ in 0..n {
    d.push(read::<i64>())
  }
  d.sort();

  let a = d[((n / 2) - 1) as usize];
  let b = d[(n / 2) as usize];
  let ans = b - a;
  println!("{}", ans);
}

