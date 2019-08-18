
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
use std::collections::BinaryHeap;

fn main() {
  let n = read::<i32>();
  let mut bh = BinaryHeap::new();
  let mut a = vec![];

  for i in 0..n {
    let a_i = read::<i32>();
    a.push(a_i);
    bh.push(a_i);
  }

  let max = bh.pop().unwrap();
  let max_second = bh.pop().unwrap();

  for i in a {
    if i == max {
      println!("{}", max_second);
    } else {
      println!("{}", max);
    }
  }
}

