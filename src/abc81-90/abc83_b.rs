
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
  let a = read::<i64>();
  let b = read::<i64>();
  let mut ans = 0;

  for i in 1..n + 1 {
    let n_str = i.to_string();
    let mut sum = 0;

    for j in n_str.chars() {
      sum += j.to_digit(10).unwrap();
    }

    if a <= sum as i64 && sum as i64 <= b {
      ans += i;
    }
  }

  println!("{}", ans);
}

