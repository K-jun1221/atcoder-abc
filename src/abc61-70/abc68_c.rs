
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
  let n = read::<i64>();
  let m = read::<i64>();
  let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

  for _ in 0..m {
    let a = read::<i64>();
    let b = read::<i64>();
    if map.contains_key(&a) {
      let current = map.get_mut(&a).unwrap();
      current.push(b)
    } else {
      map.insert(a, vec![b]);
    }

    if map.contains_key(&b) {
      let current = map.get_mut(&b).unwrap();
      current.push(a)
    } else {
      map.insert(b, vec![a]);
    }
  }

  let mut visited = vec![];
  let mut node = 1;
  let mut deep = 1;

  dfs(node, &mut visited, &map, deep);

  if visited.contains(&(n)) {
    println!("POSSIBLE");
  } else {
    println!("IMPOSSIBLE");
  }
}

fn dfs(node: i64, visited: &mut Vec<i64>, map: &HashMap<i64, Vec<i64>>, deep: i64) {
  // println!("node: {}", node);
  if 3 < deep {
    return;
  }
  if visited.contains(&node) {
    return;
  }
  visited.push(node);
  if map.contains_key(&node) {
    for (i, v) in map.get(&node).unwrap().iter().enumerate() {
      dfs(*v as i64, visited, map, deep + 1)
    }
  }

}
