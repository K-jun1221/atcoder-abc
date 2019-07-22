
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
  let n = read::<i32>();
  let mut a = vec![0; n as usize];
  for i in 0..n {
    a[i as usize] = read::<i32>();
  }

  let mut a_s: Vec<i32> = a
    .iter()
    .enumerate()
    .map(|(i, v)| (*v - (i + 1) as i32))
    .collect();
  a_s.sort();

  let med: usize = a_s[(n / 2) as usize] as usize;

  let b: i32 = a
    .iter()
    .enumerate()
    .map(|(i, v)| (*v - (med + i + 1) as i32).abs())
    .sum();

  println!("{}", b);
}

