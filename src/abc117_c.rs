
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

use std::collections::BinaryHeap;

fn main() {
  let mut n = read::<i32>();
  let m = read::<i32>();
  let mut bh_x: BinaryHeap<i32> = BinaryHeap::new();
  for _ in 0..m {
    let x = read::<i32>();
    bh_x.push(x);
  }

  let mut dist: BinaryHeap<i32> = BinaryHeap::new();

  let mut prev_v = 1000000;
  let mut index = 0;

  while true {
    let pop_x = match bh_x.pop() {
      Some(s) => s,
      None => break,
    };

    if prev_v == 1000000 {
      prev_v = pop_x;
      continue;
    } else {
      dist.push((prev_v - pop_x).abs())
    }
    prev_v = pop_x;
    index += 1;
  }

  while n - 1 > 0 {
    n -= 1;
    dist.pop();
  }

  let ans: i32 = dist.iter().sum();
  println!("{}", ans);
}

