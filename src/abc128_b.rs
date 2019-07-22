use std::collections::BTreeMap;
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

fn main() {
  let n = read::<usize>();
  let mut map: BTreeMap<String, BTreeMap<u32, usize>> = BTreeMap::new();

  for i in 0..n {
    let s = read::<String>();
    let p = read::<u32>();

    if map.contains_key(&s) {
      let mut prev_vec = map.get_mut(&s).unwrap();
      prev_vec.insert(p, i + 1);
    } else {
      let mut item_map = BTreeMap::new();
      item_map.insert(p, i + 1);
      map.insert(s, item_map);
    }
  }

  for (_, v) in &map {
    for (_, index) in v.iter().rev() {
      println!("{}", index)
    }
  }
}
