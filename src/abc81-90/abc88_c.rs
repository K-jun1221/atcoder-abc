
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
  let mut c: Vec<Vec<i64>> = vec![];
  for _ in 0..3 {
    let mut c_vec = vec![];
    for _ in 0..3 {
      c_vec.push(read::<i64>())
    }
    c.push(c_vec);
  }

  let x1 = 0;
  let y1 = c[0][0];
  let y2 = c[0][1];
  let y3 = c[0][2];
  let x2 = c[1][0] - y1;
  let x3 = c[2][0] - y1;

  if y2 + x2 == c[1][1] && y2 + x3 == c[2][1] && y3 + x2 == c[1][2] && y3 + x3 == c[2][2] {
    println!("Yes");
  } else {
    println!("No");
  }
}
