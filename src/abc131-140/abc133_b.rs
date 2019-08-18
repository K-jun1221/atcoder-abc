
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
  let n = read::<i32>();
  let d = read::<i32>();
  let mut x: Vec<Vec<i32>> = vec![];
  let mut count = 0;

  for _ in 0..n {
    let mut xi = vec![];
    for _ in 0..d {
      let xs = read::<i32>();
      xi.push(xs);
    }
    x.push(xi);
  }

  let mut ok_list = [false; 500000];

  for i in 1..401 {
    ok_list[i * i as usize] = true;
  }

  // println!("{:?}", ok_list);

  for i in 0..n {
    for j in i + 1..n {
      let a = &x[i as usize];
      let b = &x[j as usize];

      // println!("a: {:?} b:{:?}", a, b);

      let dist: i64 = a
        .iter()
        .enumerate()
        .map(|(i, v)| ((v - b[i]) * (v - b[i])) as i64)
        .sum();

      if ok_list[dist as usize] {
        count += 1;
      }

    }
  }
  println!("{}", count);
}

