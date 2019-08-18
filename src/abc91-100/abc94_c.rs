
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

fn main() {
  let n = read::<i64>();
  let mut x = vec![];
  let mut x2 = vec![];
  for _ in 0..n {
    let x_i = read::<i64>();
    x.push(x_i);
    x2.push(x_i);
  }

  x.sort();
  let center_s = x[(n / 2) as usize - 1];
  let center_l = x[(n / 2) as usize];

  for i in x2 {
    if i <= center_s {
      println!("{}", center_l);
    } else {
      println!("{}", center_s);
    }
  }
}

