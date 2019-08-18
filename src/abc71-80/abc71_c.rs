
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
  let mut a = vec![];
  let mut b = vec![];

  for _ in 0..x {
    let a = read::<i64>();
    let current = dist.entry(a).or_insert(0);
    *current += 1;
  }

  for (k, v) in dist.iter() {
    if 2 <= *v {
      a.push(k);
    }
    if 4 <= *v {
      b.push(k);
    }
  }

  if a.len() < 2 {
    println!("{}", 0)
  } else {
    a.sort();
    let mut ans = a[a.len() as usize - 1] * a[a.len() as usize - 2];
    if 1 <= b.len() {
      b.sort();
      ans = max(ans, b[b.len() - 1] * b[b.len() - 1])
    }
    println!("{}", ans)
  }

}

