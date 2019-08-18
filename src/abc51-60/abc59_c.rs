
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
use std::collections::BTreeMap;

fn main() {
  let n = read::<i64>();
  let mut vec_a = vec![];

  for i in 0..n {
    vec_a.push(read::<i64>());
  }

  let mut prev_sum = 0 as i64;
  let mut ans_plus = 0;
  let mut prev_sum2 = 0 as i64;
  let mut ans_minus = 0;

  // 最初を正にする
  if vec_a[0] <= 0 {
    prev_sum = 1;
    ans_plus = (1 - prev_sum).abs();
  } else {
    prev_sum = vec_a[0];
  }

  // 2こ目以降は流れで
  for i in 1..vec_a.len() {
    let b = vec_a[i as usize];
    if 0 < prev_sum {
      if 0 <= prev_sum + b {
        ans_plus += (1 + prev_sum).abs() + b;
        prev_sum = -1;
      } else {
        prev_sum += b;
      }
    } else if prev_sum < 0 {
      if prev_sum + b <= 0 {
        ans_plus += (1 - prev_sum).abs() - b;
        prev_sum = 1;
      } else {
        prev_sum += b;
      }
    }
  }

  // 最初を負にする
  if 0 <= vec_a[0] {
    prev_sum2 = -1;
    ans_minus = (1 + prev_sum2).abs();
  } else {
    prev_sum2 = vec_a[0];
  }
  // 2こ目以降は流れで
  for i in 1..vec_a.len() {
    let b = vec_a[i as usize];
    if 0 < prev_sum2 {
      if 0 <= prev_sum2 + b {
        ans_minus += (1 + prev_sum2).abs() + b;
        prev_sum2 = -1;
      } else {
        prev_sum2 += b;
      }
    } else if prev_sum2 < 0 {
      if prev_sum2 + b <= 0 {
        ans_minus += (1 - prev_sum2).abs() - b;
        prev_sum2 = 1;
      } else {
        prev_sum2 += b;
      }
    }
  }

  // println!("plus: {}, minus: {}", ans_minus, ans_plus);
  println!("{}", min(ans_minus, ans_plus));

}

