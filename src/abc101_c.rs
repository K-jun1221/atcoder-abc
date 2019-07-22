
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
  let k = read::<i64>();

  let mut a: Vec<i64> = vec![0; n as usize];
  for i in 0..n {
    a[i as usize] = read::<i64>();
  }

  let ans = (n - 1) / (k - 1) + (if (n - 1) % (k - 1) > 0 { 1 } else { 0 });
  println!("{}", ans);
}

