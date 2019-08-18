
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
  let a = read::<i32>();
  let b = read::<i32>();
  let c = read::<i32>();

  let tasks = vec![a, b, c];

  println!(
    "{}",
    tasks.iter().max().unwrap() - tasks.iter().min().unwrap()
  );
}

