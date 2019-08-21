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
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
  let n: usize = read();
  let k: usize = read();
  let mut neta: Vec<Vec<i64>> = vec![];
  for _ in 0..n {
    let t: i64 = read();
    let d: i64 = read();
    neta.push(vec![t, d]);
  }

  neta.sort_by_key(|x| -x[1]);
  let nokori = neta.split_off(k);
  println!("{:?}", neta);
  // let n_vec: Vec<i64> = read_n(n);
  // n_vec.sort_by_key(mut f: F)

}
