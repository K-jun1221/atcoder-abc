use std::cmp::{max, min};

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
  let s = read::<String>();

  let mut answer1 = 0;
  let mut answer2 = 0;

  for (i, v) in s.chars().enumerate() {
    if i % 2 == 0 {
      if v != '0' {
        answer1 += 1;
      }
      if v != '1' {
        answer2 += 1;
      }
    } else {
      if v != '1' {
        answer1 += 1;
      }
      if v != '0' {
        answer2 += 1;
      }
    }

  }

  println!("{}", min(answer1, answer2));
}

