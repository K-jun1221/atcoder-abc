
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
  let mut ang = vec![];
  let mut map: BTreeMap<String, i64> = BTreeMap::new();

  for _ in 0..n {
    ang.push(read::<String>());
  }

  let mut ans: i64 = 0;

  for i in ang {
    let mut item = i.chars().collect::<Vec<char>>();
    item.sort();

    let mut ang_i = String::from("");
    for i in item.iter() {
      ang_i.push(*i);
    }
    if map.contains_key(&ang_i) {
      let current = map.get_mut(&ang_i).unwrap();
      ans += *current;
      *current += 1;
    } else {
      map.insert(ang_i, 1);
    }
  }
  println!("{:?}", ans);
}
