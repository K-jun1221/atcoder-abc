
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
  let x = read::<i64>();
  // let mut memo = vec![false; x as usize + 1];
  let mut prev = 0;
  let mut max_v = 1;

  for i in 2..32 {
    prev = i;
    while prev * i <= x {
      prev = prev * i;
      max_v = max(max_v, prev);
    }
  }

  println!("{}", max_v);

  // if x < 9 {
  //   println!("4")
  // }

}

