
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

  let a_is_even = a % 2 == 0;
  let b_is_even = b % 2 == 0;

  if a_is_even == b_is_even {
    println!("{}", (a + b) / 2);
  } else {
    println!("IMPOSSIBLE");
  }

}

