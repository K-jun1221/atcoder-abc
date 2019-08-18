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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
  let n: usize = read();
  let q: usize = read();
  let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

  for _ in 0..n - 1 {
    let a = read::<i64>();
    let b = read::<i64>();
    if map.contains_key(&a) {
      let current = map.get_mut(&a).unwrap();
      current.push(b)
    } else {
      map.insert(a, vec![b]);
    }
  }

  let mut points: Vec<i64> = vec![0; n];
  for _ in 0..q {
    let a: usize = read();
    let b: i64 = read();
    points[a - 1] += b;
  }

  let mut ans: Vec<i64> = vec![0; n];
  let node = 1;
  let mut current_p = 0;
  dfs(node, &map, &points, &mut ans, &mut current_p);
  // println!("{:?}", ans);

  for i in ans {
    print!("{} ", i);
  }

}

fn dfs(
  node: i64,
  map: &HashMap<i64, Vec<i64>>,
  points: &Vec<i64>,
  ans: &mut Vec<i64>,
  current_p: &mut i64,
) {
  // println!("node: {}", node);
  *current_p += points[(node - 1) as usize];
  ans[(node - 1) as usize] = *current_p;

  if map.contains_key(&node) {
    for (i, v) in map.get(&node).unwrap().iter().enumerate() {
      let mut current_p = current_p.clone();
      dfs(*v, &map, &points, ans, &mut current_p)
    }
  }

}
