// mod custom_lib;

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

  let r = read::<u32>();
  let d = read::<u32>();
  let x2000 = read::<u32>();

  let mut prev = x2000;
  for _ in 0..10 {
    prev = r * prev - d;
    println!("{}", prev);
  }
}
