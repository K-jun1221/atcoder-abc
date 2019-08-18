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
  let s: String = read();
  let t: String = read();
  let mut dist: HashMap<char, Vec<i64>> = HashMap::new();

  for (i, c) in s.chars().enumerate() {
    if dist.contains_key(&c) {
      let current = dist.get_mut(&c).unwrap();
      current.push(i as i64);
    } else {
      dist.insert(c, vec![i as i64]);
    }
  }

  let mut ans: i64 = 0;
  let mut prev_idx = -1;

  for (i, c) in t.chars().enumerate() {
    if dist.contains_key(&c) {
      let target_vec = dist.get_mut(&c).unwrap();
      if i == 0 {
        prev_idx = target_vec[0];
        continue;
      }

      let mut current_idx = -1;
      for i in target_vec.clone() {
        if prev_idx < i {
          current_idx = i;
          break;
        }
      }

      if current_idx == -1 {
        ans += s.len() as i64;
        prev_idx = target_vec[0];
      } else {
        prev_idx = current_idx;
      }
    } else {
      ans = -1;
      break;
    }
  }

  ans += s.len() as i64;
  ans -= s.len() as i64 - prev_idx - 1;


  println!("{}", ans);

}
