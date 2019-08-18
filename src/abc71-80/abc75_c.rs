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

use std::collections::HashMap;

fn main() {
  let n = read::<usize>();
  let m = read::<usize>();
  // let mut graph: HashMap<i64, Vec<i64>> = HashMap::new();

  let mut map: Vec<Vec<usize>> = vec![vec![0; n]; n];
  let mut edges: Vec<Vec<usize>> = vec![];
  let mut cnt = 0;

  for _ in 0..m {
    let a = read::<usize>();
    let b = read::<usize>();
    map[a - 1][b - 1] = 1;
    map[b - 1][a - 1] = 1;
    edges.push(vec![a - 1, b - 1])
  }

  for i in edges {
    let a = i[0];
    let b = i[1];
    // 無効化
    map[a][b] = 0;
    map[b][a] = 0;

    // dfs
    let mut visited: Vec<usize> = vec![];
    dfs(&mut visited, 0, &map);

    // println!("{:?}", visited);
    if visited.len() != n {
      cnt += 1;
    }

    // 戻す
    map[a][b] = 1;
    map[b][a] = 1;
  }

  println!("{:?}", cnt);
}

fn dfs(visited: &mut Vec<usize>, node: usize, map: &Vec<Vec<usize>>) {
  if visited.contains(&node) {
    return;
  }
  visited.push(node);
  for (i, v) in map[node].iter().enumerate() {
    if *v == 1 {
      dfs(visited, i, map)
    }
  }
}

