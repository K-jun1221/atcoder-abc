
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
  let mut ans = 0;
  ans += (n / 11) * 2;
  if n % 11 == 0 {
    ans += 0;
  } else if n % 11 <= 6 {
    ans += 1;
  } else {
    ans += 2;
  }

  println!("{}", ans);
}

