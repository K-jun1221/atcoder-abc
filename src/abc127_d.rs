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

pub fn read_n<T: FromStr>(n: u32) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: u32, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

use std::cmp;

use std::collections::BTreeMap;
use std::collections::BinaryHeap;
fn main() {

  let n = read::<u32>();
  let m = read::<i64>();
  let a = read_n::<i64>(n);
  let mut bh_a: BinaryHeap<i64> = BinaryHeap::new();

  let mut sum = 0;

  for i in a {
    bh_a.push(-i);
  }

  let mut bmap: BTreeMap<i64, i64> = BTreeMap::new();

  for _ in 0..m {
    let b = read::<i64>();
    let c = read::<i64>();
    if bmap.contains_key(&c) {
      let mut temp = bmap.get_mut(&c).unwrap();
      *temp += b;
    } else {
      bmap.insert(c, b);
    }
  }

  let mut done = false;

  for (idx, val) in bmap.iter().rev() {

    for i in 0..*val {
      let bh_a_pop = match bh_a.pop() {
        Some(s) => s,
        None => {
          done = true;
          break;
        }
      };

      if -bh_a_pop >= *idx {
        bh_a.push(bh_a_pop);
        done = true;
        break;
      } else {
        bh_a.push(-idx);
      }
    }

    if done {
      break;
    }
  }

  let sum_bh_a: i64 = bh_a.iter().sum();
  println!("{}", -sum_bh_a);

}

fn get_maxinum(bmap: &mut BTreeMap<i64, i64>) -> i64 {
  let mut found_num = 0;
  for (idx, val) in bmap.iter().rev() {
    if *val > 0 {
      found_num = *idx;
      break;
    }
  }
  found_num
}

fn subscribe_bmap(bmap: &mut BTreeMap<i64, i64>, used_index: &i64) {
  if bmap.contains_key(used_index) {
    let mut temp = bmap.get_mut(used_index).unwrap();
    *temp -= 1;
  }
}
