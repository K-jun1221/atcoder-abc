
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
  let m = read::<i32>();
  let mut a = vec![0; m as usize];
  for i in 0..m {
    a[i as usize] = read::<i32>();
  }

  println!("{}", a.iter().max().unwrap() - a.iter().min().unwrap());
}

