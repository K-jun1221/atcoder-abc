
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
  let n = read::<usize>();
  let m = read::<usize>();
  let mut map: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];

  for _ in 0..m {
    let a = read::<usize>();
    let b = read::<usize>();
    map[a][b] = 1;
    map[b][a] = 1;
  }

  let mut visited: Vec<usize> = vec![];
  let mut node = 1;
  let mut cnt = 0;

  dfs(node, &mut visited, &map, n, &mut cnt);
  println!("{}", cnt);
}


fn dfs(node: usize, visited: &mut Vec<usize>, map: &Vec<Vec<usize>>, n: usize, cnt: &mut i64) {
  if visited.contains(&node) {
    return;
  }
  visited.push(node);
  if visited.len() == n {
    *cnt += 1;
    return;
  }

  for (i, v) in map[node].iter().enumerate() {
    if *v == 1 {
      let mut copy_visited = visited.clone();
      dfs(i, &mut copy_visited, map, n, cnt);
    }
  }
}