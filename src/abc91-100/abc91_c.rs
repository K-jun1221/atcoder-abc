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
  let n = read::<i64>();
  let mut red = vec![];
  let mut blue = vec![];
  let mut ans = 0;

  for _ in 0..n {
    let a = read::<i64>();
    let b = read::<i64>();
    red.push(vec![a, b])
  }
  for _ in 0..n {
    let a = read::<i64>();
    let b = read::<i64>();
    blue.push(vec![a, b])
  }

  blue.sort_by_key(|x| x[0]);
  red.sort_by_key(|x| -x[1]);

  for (i, v) in blue.iter().enumerate() {
    // let mut dib = i;
    let mut dir = -1 as i64;
    for (ii, iv) in red.iter().enumerate() {
      if iv[1] < v[1] && iv[0] < v[0] {
        dir = ii as i64;
        ans += 1;
        break;
      }
    }
    if dir != -1 {
      red.remove(dir as usize);
    }
  }

  println!("{}", ans);
}