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

fn main() {
  let n: usize = read();
  let mut a: Vec<f64> = read_n(n);
  let a_len = a.len();

  for i in 0..a_len - 1 {
    a.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = a.remove(0);
    let second_min = a.remove(0);
    a.push((min + second_min) / 2.0);
  }


  println!("{:?}", a[0]);
}
