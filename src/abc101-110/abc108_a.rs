
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
  let mut ans = 0;

  if a % 2 == 0 {
    ans = (a / 2).pow(2)
  } else {
    ans = ((a - 1) / 2) * ((a + 1) / 2)
  }

  println!("{}", ans);

}

