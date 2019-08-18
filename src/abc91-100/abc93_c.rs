
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
  let mut a = read::<i64>();
  let mut b = read::<i64>();
  let mut c = read::<i64>();
  let a_is_even = a % 2 == 0;
  let b_is_even = b % 2 == 0;
  let c_is_even = c % 2 == 0;
  let mut ans = 0;

  if !a_is_even && !b_is_even && !c_is_even {
    a -= 1;
    b -= 1;
    c -= 1;
  } else {
    if !a_is_even {
      ans += 1;
      a -= 1;
    }
    if !b_is_even {
      ans += 1;
      b -= 1;
    }
    if !c_is_even {
      ans += 1;
      c -= 1;
    }
  }


  let max_v = max(a, max(b, c));
  println!(
    "{}",
    ans + ((max_v - b) / 2) + ((max_v - c) / 2) + ((max_v - a) / 2)
  );
}

