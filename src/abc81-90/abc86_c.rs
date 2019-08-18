
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

fn gcd(a: i64, b: i64) -> i64 {
  if a % b == 0 {
    b
  } else {
    gcd(b, a % b)
  }
}

fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}

fn main() {
  let n = read::<i64>();
  let mut vec: Vec<Vec<i64>> = vec![];
  let mut s_x = 0;
  let mut s_y = 0;
  let mut time = 0;
  let mut ans = "Yes";

  for _ in 0..n {
    let t = read::<i64>();
    let x = read::<i64>();
    let y = read::<i64>();
    vec.push(vec![t, x, y]);
  }

  for i in vec {
    let t = i[0];
    let x = i[1];
    let y = i[2];
    let d_x = (s_x - x).abs();
    let d_y = (s_y - y).abs();
    let d_time = (time - t).abs();

    if (d_x + d_y) % 2 != d_time % 2 || d_time < d_x + d_y {
      ans = "No";
      break;
    }
    s_x = x;
    s_y = y;
    time = t;
  }

  println!("{}", ans);
}
