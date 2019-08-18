
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
  let mut p = vec![];
  for _ in 0..n {
    p.push(read::<i64>())
  }
  let mut cnt = 0;

  for i in 0..n - 2 {
    let idx = i as usize;
    let one = p[idx];
    let two = p[idx + 1];
    let three = p[idx + 2];

    if one < two && two < three || three < two && two < one {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}

