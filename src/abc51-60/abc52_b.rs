
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
  let b = read::<String>();
  let a = read::<String>();
  let mut x = 0;
  let mut max_x = 0;
  for c in a.chars() {
    if c == 'I' {
      x += 1;
      max_x = max(max_x, x)
    } else {
      x -= 1;
    }
  }

  println!("{}", max_x);
}

