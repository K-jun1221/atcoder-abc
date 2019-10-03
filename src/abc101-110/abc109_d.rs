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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
  let mut h: usize = read();
  let mut w: usize = read();
  let mut map = vec![];

  for i in 0..h {
    let mut a: Vec<i64> = read_n(w);
    map.push(a);
  }

  let mut amari = 0;
  let mut moves: Vec<Vec<usize>> = vec![];

  for i in 0..h {
    if i % 2 == 0 {
      for j in 0..w {
        if amari != 0 {
          map[i][j] += amari;
          moves.push(vec![i + 1, j + 1]);
        }

        if map[i][j] % 2 == 1 {
          amari = 1;
          map[i][j] -= 1;
          moves.push(vec![i + 1, j + 1]);
        } else {
          amari = 0;
        }
      }
    } else {
      for j in (0..w).rev() {
        if amari != 0 {
          map[i][j] += amari;
          moves.push(vec![i + 1, j + 1]);
        }
        if map[i][j] % 2 == 1 {
          map[i][j] -= 1;
          amari = 1;
          moves.push(vec![i + 1, j + 1]);
        } else {
          amari = 0;
        }
      }
    }
  }
  println!("{}", (moves.len() - (moves.len() % 2)) / 2);
  for i in 0..moves.len() - (moves.len() % 2) {
    if i % 2 == 1 {
      continue;
    }
    println!(
      "{} {} {} {}",
      moves[i][0],
      moves[i][1],
      moves[i + 1][0],
      moves[i + 1][1]
    );
  }
}