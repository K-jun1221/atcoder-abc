
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
  let n = read::<i64>();
  let m = read::<i64>();

  let mut students: Vec<Vec<i64>> = vec![];
  let mut cp: Vec<Vec<i64>> = vec![];
  let mut ans = vec![];

  for _ in 0..n {
    let x = read::<i64>();
    let y = read::<i64>();
    students.push(vec![x, y])
  }
  for _ in 0..m {
    let x = read::<i64>();
    let y = read::<i64>();
    cp.push(vec![x, y])
  }

  for i in students {
    let mut close_cp = 0;
    let mut min_dist = 1000000000000000;
    for (idx, j) in cp.iter().enumerate() {
      let dist = (i[0] - j[0]).abs() + (i[1] - j[1]).abs();
      if dist < min_dist {
        min_dist = dist;
        close_cp = idx + 1;
      }
    }

    ans.push(close_cp);
  }
  for i in ans {
    println!("{}", i);
  }
}

