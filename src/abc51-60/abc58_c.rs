
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
  let mut dist: BTreeMap<char, i64> = BTreeMap::new();
  let mut s_vec = vec![];

  for _ in 0..n {
    let s = read::<String>();
    s_vec.push(s);
  }

  let first = &s_vec[0];
  for c in first.chars() {
    let current = dist.entry(c).or_insert(0);
    *current += 1;
  }

  for i in &s_vec {
    let mut map: BTreeMap<char, i64> = BTreeMap::new();

    for (k, v) in dist.iter() {
      map.insert(*k, 0);
    }

    for c in i.chars() {
      let current = map.entry(c).or_insert(0);
      *current += 1;
    }


    for (k, v) in map.iter() {
      if dist.contains_key(k) {
        let current = dist.entry(*k).or_insert(0);
        if v < current {
          *current = *v;
        }
      }
    }
  }

  let mut ans = String::from("");

  for (k, v) in dist.iter() {
    for _ in 0..*v {
      ans.push(*k);
    }
  }
  println!("{}", ans);

}

