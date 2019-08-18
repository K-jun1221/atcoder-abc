
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
  let mut s = read::<String>();
  let mut a_idx = 1000000000;
  let mut z_idx = 0;

  for (i, v) in s.chars().enumerate() {
    if v == 'A' {
      a_idx = min(i, a_idx);
    }
    if v == 'Z' {
      z_idx = max(i, z_idx);
    }
  }
  println!("{}", z_idx - a_idx + 1);
}

