
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
  let d = read::<i64>();
  let n = read::<i64>();
  if n == 100 {
    println!(
      "{}",
      ((100 as u32).pow(d as u32)) * n as u32 + (100 as u32).pow(d as u32)
    );
  } else {
    println!("{}", (100 as u32).pow(d as u32) * n as u32);
  }

}