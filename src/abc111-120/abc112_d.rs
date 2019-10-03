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
use std::io::{stdout, BufWriter, Write};

fn divisors(N: i64) -> HashSet<i64> {
  let mut ds = vec![];
  let mut d = 1;
  while d * d <= N {
    if N % d == 0 {
      ds.push(d);
      ds.push(N / d);
    }
    d += 1;
  }
  let uniq: HashSet<i64> = ds.into_iter().collect();
  uniq
}

fn main() {
  let mut n: i64 = read();
  let mut points: Vec<(i64, i64)> = vec![];
  let mut max_dist = 0;

  let out = stdout();
  let mut out = BufWriter::new(out.lock());;

  for _ in 0..n {
    let mut x: i64 = read();
    let mut y: i64 = read();
    points.push((x, y));

    max_dist = max(max_dist, x.abs() + y.abs());
  }

  if points
    .iter()
    .all(|item| (item.1.abs() + item.0.abs()) % 2 == 1)
    || points
      .iter()
      .all(|item| (item.1.abs() + item.0.abs()) % 2 == 0)
  {
    writeln!(out, "{}", max_dist).unwrap();
    for _ in 0..max_dist {
      write!(out, "{} ", 1).unwrap();
    }
    writeln!(out, "").unwrap();
    for i in points {
      let x = i.0;
      let y = i.1;
      let mut commands: String = String::from("");
      if x.abs() + y.abs() < max_dist {
        for _ in 0..((max_dist - x.abs() + y.abs()) / 2) {
          write!(out, "R").unwrap();
          write!(out, "L").unwrap();
        }
      }
      for _ in 0..x.abs() {
        if x < 0 {
          write!(out, "L").unwrap();
        } else {
          write!(out, "R").unwrap();
        }
      }

      for _ in 0..y.abs() {
        if y < 0 {
          write!(out, "D").unwrap();
        } else {
          write!(out, "U").unwrap();
        }
      }

      writeln!(out, "").unwrap();
    }
  } else {
    writeln!(out, "-1").unwrap();
  }

  // println!("{}", ans)
}