
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
  let mut sx = read::<i64>();
  let mut sy = read::<i64>();
  let mut tx = read::<i64>();
  let mut ty = read::<i64>();
  let mut ans = String::from("");

  let abs_y = ty - sy;
  let abs_x = tx - sx;
  // 1往路
  for i in 0..abs_y {
    ans.push('U')
  }
  for i in 0..abs_x {
    ans.push('R')
  }

  // 1帰り
  ans.push('R');
  for i in 0..abs_y + 1 {
    ans.push('D')
  }
  for i in 0..abs_x + 1 {
    ans.push('L')
  }
  ans.push('U');

  // 2行き
  for i in 0..abs_x {
    ans.push('R')
  }
  for i in 0..abs_y {
    ans.push('U')
  }

  // 2帰り
  ans.push('U');
  for i in 0..abs_x + 1 {
    ans.push('L')
  }
  for i in 0..abs_y + 1 {
    ans.push('D')
  }
  ans.push('R');

  println!("{}", ans);
}

