
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
  let n = read::<u32>();
  let x = read::<i32>();
  let mut x_vec = vec![];

  for _ in 0..n {
    let x_i = read::<i32>();
    let x_abs = (x - x_i).abs();
    x_vec.push(x_abs as u32);
  }
  let mut ans = x_vec[0];
  for i in x_vec {
    ans = gcd(ans, i);
  }

  println!("{}", ans);
}


fn gcd(a: u32, b: u32) -> u32 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}
