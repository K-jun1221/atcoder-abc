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
  let k = read::<i64>();
  let mut sum_s = vec![];
  let s = read::<String>();

  // 連続数で配列
  let mut now = '1';
  let mut cnt = 0;
  for i in s.chars() {
    if i != now {
      sum_s.push(cnt);
      cnt = 1;
      now = if now == '1' { '0' } else { '1' }
    } else {
      cnt += 1;
    }
  }
  sum_s.push(cnt);
  if sum_s.len() % 2 == 0 {
    sum_s.push(0);
  }

  let length = (k * 2 + 1) as usize;
  let sum_len = sum_s.len() as i64;
  let mut ans = 0;
  let mut block = 0;
  for i in 0..min(length as i64, sum_len) {
    block += sum_s[i as usize];
  }

  ans = block;

  for i in 0..sum_len - length as i64 {
    if i % 2 == 1 {
      continue;
    }
    let i_size = i as usize;
    let block_f = sum_s[i_size];
    let block_s = sum_s[i_size + 1];

    let block_n = sum_s[i_size + length];
    let block_n2 = sum_s[i_size + 1 + length];

    // println!(
    //   "first: {}, second: {}, last1: {}, last2: {}",
    //   block_f, block_s, block_n, block_n2
    // );
    block += block_n + block_n2 - block_s - block_f;
    // println!("block: {}", block);
    ans = max(ans, block)
  }
  println!("{}", ans);

}

