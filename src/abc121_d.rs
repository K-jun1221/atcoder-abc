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
use std::collections::VecDeque;

fn main() {
  let a = read::<i64>();
  let b = read::<i64>();

  let mut ans = String::from("");

  // aまでの排他的論理和を求める
  let v2 = calc(b, 0);
  let v = calc(a - 1, v2.len());

  // println!("{:?}", v);
  // println!("{:?}", v2);

  for i in 0..v2.len() {
    if v[i] + v2[i] == 1 {
      ans.push('1')
    } else {
      ans.push('0')
    }
  }

  // println!("{}", ans);
  let intval = isize::from_str_radix(&ans, 2).unwrap();
  println!("{}", intval);
}


fn calc(a: i64, size: usize) -> VecDeque<i64> {
  let mut v = VecDeque::new();
  if a % 4 == 0 {
    v.push_front(0);
    v.push_front(0);
  } else if a % 4 == 1 {
    v.push_front(1);
    v.push_front(0);
  } else if a % 4 == 2 {
    v.push_front(1);
    v.push_front(1);
  } else {
    v.push_front(0);
    v.push_front(0);
  }

  let mut w = 4;
  while a % w != a {
    w = w * 2;
    if a % w < w / 2 || (a % w) % 2 == 1 {
      v.push_front(0);
    } else {
      v.push_front(1);
    }
  }

  if size != 0 {
    let v_len = v.len();

    for _ in 0..(size - v_len) {
      v.push_front(0);
    }
  }


  return v;
}

