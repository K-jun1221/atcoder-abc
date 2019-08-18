
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


use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;

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

fn main() {
  let n = read::<i64>();
  let mut middle_map: BTreeMap<i64, i64> = BTreeMap::new();

  let mut top_vec = vec![];
  let mut bottom_vec = vec![];

  for _ in 0..n {
    let a = read::<usize>();
    top_vec.push(a);
  }

  for _ in 0..n {
    let b = read::<i64>();
    *middle_map.entry(b).or_insert(0) += 1;
  }

  for _ in 0..n {
    let c = read::<usize>();
    bottom_vec.push(c);
  }

  top_vec.sort();
  bottom_vec.sort();
  let mut ans = 0;

  for (k, v) in middle_map.iter() {
    ans += top_vec.lower_bound(&(*k as usize)) as i64
      * *v
      * (n - bottom_vec.upper_bound(&(*k as usize)) as i64);
  }

  println!("{}", ans);
}