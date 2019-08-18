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

fn main() {
  let a: usize = read();
  let b: usize = read();
  let q: usize = read();
  let mut a_vec: Vec<i64> = read_n(a);
  let mut b_vec: Vec<i64> = read_n(b);
  let mut q_vec: Vec<i64> = read_n(q);

  a_vec.sort();
  b_vec.sort();


  for i in q_vec {
    let mut answers = vec![];
    // aを先に回る
    let points = close_points(&a_vec, i);
    for j in points {
      let points2 = close_points(&b_vec, j);
      for k in points2 {
        answers.push((i - j).abs() + (j - k).abs())
      }
    }

    // bを先に回る
    let points = close_points(&b_vec, i);
    for j in points {
      let points2 = close_points(&a_vec, j);
      for k in points2 {
        answers.push((i - j).abs() + (j - k).abs())
      }
    }
    println!("{}", answers.iter().min().unwrap());
  }
}

fn close_points(a_vec: &Vec<i64>, i: i64) -> Vec<i64> {
  let mut return_value = vec![];
  let mut a_idx = a_vec.lower_bound(&i);
  if a_idx == a_vec.len() {
    a_idx -= 1;
    return_value.push(a_vec[a_idx]);
  } else {
    return_value.push(a_vec[a_idx]);
    if a_idx != 0 {
      return_value.push(a_vec[a_idx - 1]);
    }
  }
  return return_value;
}

use std::cmp::Ordering;

pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut low = 0;
    let mut high = self.len();

    while low != high {
      let mid = (low + high) / 2;
      match self[mid].cmp(x) {
        Ordering::Less => {
          low = mid + 1;
        }
        Ordering::Equal | Ordering::Greater => {
          high = mid;
        }
      }
    }
    low
  }
  fn upper_bound(&self, x: &T) -> usize {
    let mut low = 0;
    let mut high = self.len();

    while low != high {
      let mid = (low + high) / 2;
      match self[mid].cmp(x) {
        Ordering::Less | Ordering::Equal => {
          low = mid + 1;
        }
        Ordering::Greater => {
          high = mid;
        }
      }
    }
    low
  }
}