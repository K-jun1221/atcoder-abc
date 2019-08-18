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

fn main() {
  let x = read::<i64>();
  let y = read::<i64>();
  let z = read::<i64>();
  let k = read::<i64>();
  let mut a_vec = vec![];
  let mut b_vec = vec![];
  let mut c_vec = vec![];
  let mut anss = vec![];

  for _ in 0..x {
    a_vec.push(read::<i64>())
  }

  for _ in 0..y {
    b_vec.push(read::<i64>())
  }

  for _ in 0..z {
    c_vec.push(read::<i64>())
  }

  // x*yで計算して上位kだけ取り出す
  let mut ab_vec = vec![];
  for b in b_vec {
    for a in &a_vec {
      ab_vec.push(a + b)
    }
  }

  ab_vec.sort();
  ab_vec.reverse();
  if k < ab_vec.len() as i64 {
    ab_vec.split_off(k as usize);
  }

  // zを加えて計算して上位kだけ取り出す
  for i in ab_vec {
    for c in &c_vec {
      anss.push(i + c);
    }
  }

  anss.sort();
  anss.reverse();

  if k < anss.len() as i64 {
    anss.split_off(k as usize);
  }


  for i in anss {
    println!("{}", i);
  }
}

