
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
  let s = read::<String>();

  let mut a: i32 = 0;
  let mut b: i32 = 0;
  let mut c: i32 = 0;
  let mut ans: i32 = 1000;

  for (i, v) in s.chars().enumerate() {
    if (i == 0) {
      a = v.to_string().parse::<i32>().unwrap();
    } else if (i == 1) {
      b = v.to_string().parse::<i32>().unwrap();
    } else {
      c = v.to_string().parse::<i32>().unwrap();
      ans = min(ans, (753 - (a * 100 + b * 10 + c)).abs());
      a = b;
      b = c;
    }
  }

  println!("{}", ans);
}

