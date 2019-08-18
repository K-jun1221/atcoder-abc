
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
  let mut s_vec = vec![];
  for i in s.chars() {
    s_vec.push(i);
  }
  let s_len = s_vec.len();
  let mut ans = vec![0; s_len];

  // 左から
  let mut cnt = 1;
  for i in 0..s_len - 1 {
    let current = s_vec[i];
    let next = s_vec[i + 1];
    if current == 'R' && next == 'L' {
      if cnt % 2 == 0 {
        ans[i + 1] += cnt / 2;
        ans[i] += cnt / 2;
      } else {
        ans[i + 1] += (cnt - 1) / 2;
        ans[i] += (cnt + 1) / 2;
      }
    }
    if current == 'R' && next == 'R' {
      cnt += 1;
    }
    if current == 'L' {
      cnt = 1;
    }
  }

  // 右から
  cnt = 1;
  for i in 1..s_len {
    let idx = s_len - i;
    let current = s_vec[idx];
    let next = s_vec[idx - 1];

    if current == 'L' && next == 'R' {
      if cnt % 2 == 0 {
        ans[idx - 1] += cnt / 2;
        ans[idx] += cnt / 2;
      } else {
        ans[idx] += (cnt + 1) / 2;
        ans[idx - 1] += (cnt - 1) / 2;
      }
    }
    if current == 'L' && next == 'L' {
      cnt += 1;
    }
    if current == 'R' {
      cnt = 1;
    }
  }

  for (i, v) in ans.iter().enumerate() {
    if i + 1 == s_len {
      println!("{}", v);
    } else {
      print!("{} ", v);
    }
  }
}

