
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
  let mut a = read::<i64>();
  let mut b = read::<i64>();
  let mut ans = "";

  if a == b {
    ans = "Draw"
  } else if a == 1 && b != 1 {
    ans = "Alice"
  } else if a != 1 && b == 1 {
    ans = "Bob"
  } else if a < b {
    ans = "Bob"
  } else {
    ans = "Alice"
  }

  println!("{}", ans);
}

