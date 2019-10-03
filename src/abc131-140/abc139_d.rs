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

use std::collections::BTreeMap;
use std::collections::VecDeque;

fn main() {

  let n: i64 = read();
  let mut bh: BTreeMap<i64, VecDeque<i64>> = BTreeMap::new();

  let mut ans = 0;
  for i in 0..n {
    let mut v = VecDeque::new();
    for _ in 0..n - 1 {
      let item: i64 = read();
      v.push_back(item)
    }
    bh.insert(i, v);
  }


  while true {
    ans += 1;
    let mut b = vec![];
    for i in 0..n {
      let current = bh.get_mut(&i).unwrap();
      let b_i = match current.pop_front() {
        Some(s) => s,
        _ => -1,
      };
      if b_i <= b.len() as i64 && b_i != -1 && b[(b_i - 1) as usize] == i + 1 {
        b.push(0);
        b[(b_i - 1) as usize] = 0;
      } else {
        b.push(b_i)
      }
    }

    // TODO 全部-1で終了
    if b.iter().all(|x| *x == -1) {
      break;
    }

    // TODO 0が一つもなかったらans=-1になる
    if b.iter().all(|x| *x != 0) {
      ans = -1;
      break;
    }

    for (i, v) in b.iter().enumerate() {
      let i_int = i as i64;
      if *v != 0 && *v != -1 {
        let current = bh.get_mut(&i_int).unwrap();
        current.push_front(*v);
      }
    }
  }

  if ans == -1 {
    println!("{}", ans);
  } else {
    println!("{}", ans - 1);
  }

}
