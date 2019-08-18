
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
// use std::collections::BinaryHeap;

fn main() {
  let n = read::<i64>();
  let mut four = 0;
  let mut two = 0;
  for _ in 0..n {
    let a = read::<i64>();
    if a % 4 == 0 {
      four += 1;
    } else if a % 2 == 0 {
      two += 1;
    }
  }

  if 0 < four && two == 0 {
    println!("{}", if 2 * four + two + 1 < n { "No" } else { "Yes" });
  } else if 0 < four {
    println!("{}", if 2 * four + two < n { "No" } else { "Yes" });
  } else {
    println!("{}", if two < n { "No" } else { "Yes" });
  }
}
