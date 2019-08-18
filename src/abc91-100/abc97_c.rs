
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
  let x = read::<String>();
  let k = read::<i64>();
  let mut map: BTreeMap<&str, i64> = BTreeMap::new();
  for i in 0..x.len() {
    for j in i..i + k as usize {
      let min_v = min(x.len(), j + 1);
      let a = &x[i..min_v];
      map.entry(&a).or_insert(0);
    }
  }

  let (k, _) = map.iter().nth(k as usize - 1).unwrap();
  println!("{}", k);
}

