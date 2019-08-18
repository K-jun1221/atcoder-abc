
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
use std::collections::BinaryHeap;

fn main() {
  let n = read::<i64>();
  let m = read::<i64>();
  // let mut map: BTreeMap<i64, BinaryHeap<i64>> = BTreeMap::new();
  let mut vec = vec![];

  for _ in 0..n {
    let a = read::<i64>();
    let b = read::<i64>();
    vec.push(vec![a, b]);
  }

  vec.sort_by(|a, b| a[0].cmp(&b[0]));
  let mut q = BinaryHeap::new();
  let mut idx = 0;
  let mut ans = 0;

  for k in 1..m + 1 {
    while idx < n && vec[idx as usize][0] <= k {
      q.push(vec[idx as usize][1]);
      idx += 1;
    }

    ans += match q.pop() {
      Some(b) => b,
      _ => 0,
    };

  }
  println!("{:?}", ans);

}

