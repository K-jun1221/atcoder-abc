
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
  let mut up = vec![];
  let mut down = vec![];
  let mut memo: Vec<i64> = vec![0; n as usize];

  for _ in 0..n {
    up.push(read::<i64>())
  }
  for _ in 0..n {
    down.push(read::<i64>())
  }

  for i in 0..n {
    let up_sub = &up[..i as usize + 1];
    let down_sub = &down[i as usize..];
    let mut total = 0;
    for i in up_sub {
      total += *i
    }
    for i in down_sub {
      total += *i
    }
    memo[i as usize] = total;
  }

  println!("{:?}", memo.iter().max().unwrap());
}
