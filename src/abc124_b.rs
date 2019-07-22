use std::cmp::{max, min};

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

fn main() {
  let n = read::<i32>();
  let mut a = vec![];

  for _ in 0..n {
    let v = read::<i32>();
    a.push(v);
  }
  let mut m = 0;
  let mut counter = 0;

  for i in a {
    if (m <= i) {
      m = i;
      counter += 1;
    }
  }

  println!("{}", counter);
}

