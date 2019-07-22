// mod custom_lib;

use std::cmp;

fn main() {
  let p = read::<u32>();
  let q = read::<u32>();
  let r = read::<u32>();
  let answer = cmp::min(p + q, cmp::min(q + r, r + p));
  println!("{}", answer);
}

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