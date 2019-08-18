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
  let mut dist: HashMap<String, i64> = HashMap::new();
  let n = read::<i64>();
  for _ in 0..n {
    let word = read::<String>();
    let current = dist.entry(word).or_insert(0);
    *current += 1;
  }

  let m = read::<i64>();
  for _ in 0..m {
    let word = read::<String>();
    let current = dist.entry(word).or_insert(0);
    *current -= 1;
  }

  let mut ans_max = 0;
  let mut ans = "";
  for (k, v) in dist.iter() {
    if ans_max < *v {
      ans = k;
      ans_max = *v;
    }
  }

  println!("{}", ans_max);
}


