
use std::io::*;
use std::str::FromStr;

use std::cmp::{max, min};

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
  let n = read::<i32>();
  let a = read::<i32>();
  let b = read::<i32>();
  let c = read::<i32>();
  let mut l = vec![];

  for _ in 0..n {
    let value = read::<i32>();
    l.push(value);
  }

  println!("{}", dfs(0, 0, 0, 0, n, a, b, c, &l))
}

fn dfs(
  cur: i32,
  a_v: i32,
  b_v: i32,
  c_v: i32,
  n: i32,
  a: i32,
  b: i32,
  c: i32,
  l: &Vec<i32>,
) -> i32 {
  if cur == n {
    return if min(a_v, min(b_v, c_v)) > 0 {
      (a_v - a).abs() + (b_v - b).abs() + (c_v - c).abs() - 30
    } else {
      (10 as i32).pow(9)
    };
  }

  let ret0 = dfs(cur + 1, a_v, b_v, c_v, n, a, b, c, l);
  let ret1 = dfs(cur + 1, a_v + l[cur as usize], b_v, c_v, n, a, b, c, l) + 10;
  let ret2 = dfs(cur + 1, a_v, b_v + l[cur as usize], c_v, n, a, b, c, l) + 10;
  let ret3 = dfs(cur + 1, a_v, b_v, c_v + l[cur as usize], n, a, b, c, l) + 10;
  return min(min(ret0, ret1), min(ret2, ret3));
}
