
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
  let s = read::<String>();
  let mut memo = vec![0; n as usize];
  let s_vec: Vec<char> = s.chars().collect();

  // 右から
  let mut point = 0;
  for i in 0..n - 1 {
    let i_usize = i as usize;
    let current = s_vec[i_usize];
    let next = s_vec[i_usize + 1];

    if current == 'E' {
      point += 1;
    }

    if current == 'E' && next == 'W' {
      memo[i_usize] += point;
      memo[i_usize + 1] += point;
    }

    if i == n - 2 && current == 'E' && next == 'E' {
      memo[i_usize + 1] += point + 1;
    }
  }

  // 左から
  let mut point2 = 0;
  for i in 0..n - 1 {
    let back_i = n - i - 1;
    let back_i_usize = back_i as usize;
    let current = s_vec[back_i_usize];
    let next = s_vec[back_i_usize - 1];

    if current == 'W' {
      point2 += 1;
    }

    if current == 'W' && next == 'E' {
      memo[back_i_usize] += point2;
      memo[back_i_usize - 1] += point2;
    }

    if i == n - 2 && current == 'W' && next == 'W' {
      memo[back_i_usize - 1] += point2 + 1;
    }
  }

  let max_cnt = memo.iter().max().unwrap();
  println!("{}", n - max_cnt);
}

