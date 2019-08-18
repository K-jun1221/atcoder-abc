
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
  let x = read::<i64>();
  let mut dist: HashMap<i64, i64> = HashMap::new();
  let mut max_v = 0;
  let mut min_v = 0;
  let mut ans = 0;

  for i in 0..x {
    let x_i = read::<i64>();
    let current = dist.entry(x_i).or_insert(0);
    *current += 1;
    max_v = max(max_v, x_i);
    min_v = min(min_v, x_i);
  }

  for i in min_v..max_v + 1 {
    let mut sum: i64 = 0;
    sum += match dist.get(&(i + 1)) {
      Some(s) => *s,
      None => 0,
    };
    sum += match dist.get(&(i)) {
      Some(s) => *s,
      None => 0,
    };
    sum += match dist.get(&(i - 1)) {
      Some(s) => *s,
      None => 0,
    };
    ans = max(ans, sum);
  }

  println!("{}", ans);
}

