
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
use std::collections::BinaryHeap;

fn main() {
  let n = read::<i64>();
  let mut bh: BinaryHeap<i64> = BinaryHeap::new();

  let mut six_pow = 6;
  let mut nine_pow = 9;

  // 候補の書き出し
  while six_pow <= n {
    bh.push(-six_pow);
    six_pow *= 6;
  }

  while nine_pow <= n {
    bh.push(-nine_pow);
    nine_pow *= 9;
  }


  // DPで解く
  let mut dp: Vec<Vec<i64>> = vec![];


  let mut first_row = vec![];
  for i in 0..n + 1 {
    first_row.push(i);
  }
  dp.push(first_row);

  for _ in 0..bh.len() {
    dp.push(vec![0; n as usize + 1])
  }

  let bh_len = bh.len();

  for i in 1..bh.len() + 1 {
    let i_usize = i as usize;
    let a = -bh.pop().unwrap() as i64;
    for j in 1..n + 1 {
      let j_usize = j as usize;
      if (0 < j_usize as i64 - a) {
        dp[i_usize][j_usize] = min(
          dp[i_usize - 1][j_usize],
          dp[i_usize][j_usize - a as usize] + 1,
        );
      } else {
        dp[i_usize][j_usize] = dp[i_usize - 1][j_usize]
      }
    }
  }
  println!("{:?}", dp[bh_len as usize][n as usize]);
}

