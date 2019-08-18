
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
  let mut f: Vec<Vec<i64>> = vec![];
  let mut p: Vec<Vec<i64>> = vec![];
  let mut ans = -1000000;

  for _ in 0..n {
    let mut f_i = vec![];
    for _ in 0..10 {
      f_i.push(read::<i64>());
    }
    f.push(f_i);
  }

  for _ in 0..n {
    let mut p_i = vec![];
    for _ in 0..11 {
      p_i.push(read::<i64>());
    }
    p.push(p_i);
  }

  let two: u32 = 2;
  for i in 1..two.pow(10) + 1 {
    let result: String = format!("{:0>keta$b}", i, keta = 10 as usize);
    let choice: Vec<char> = result.chars().collect();
    let mut total = 0;

    for j in 0..n {
      let j_size = j as usize;
      let mut cnt = 0;
      for (idx, v) in f[j_size].iter().enumerate() {
        if choice[idx] == '1' && *v == 1 {
          cnt += 1;
        }
      }
      total += p[j_size][cnt];
    }
    ans = max(ans, total);
  }
  println!("{}", ans);
}