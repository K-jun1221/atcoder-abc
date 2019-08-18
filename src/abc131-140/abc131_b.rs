
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
  let mut n = read::<i64>();
  let mut l = read::<i64>();

  let mut vec_l = vec![];
  let mut target = 1000000000;

  for i in 1..n + 1 {
    vec_l.push(i + l - 1);
    if (i + l - 1).abs() < target {
      target = (i + l - 1).abs()
    }
  }

  let sum = vec_l.iter().sum::<i64>();
  if sum < 0 {
    println!("{:?}", sum + target);
  } else {
    println!("{:?}", sum - target);
  }

}
