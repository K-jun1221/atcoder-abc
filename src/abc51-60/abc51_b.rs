
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
  let mut k = read::<i64>();
  let mut s = read::<i64>();
  let mut cnt = 0;

  for i in 0..k + 1 {
    for j in 0..k + 1 {
      if s - i - j <= k && 0 <= s - i - j {
        cnt += 1;
      }
    }
  }

  println!("{}", cnt);
}

