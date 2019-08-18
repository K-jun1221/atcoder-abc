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
  let n: usize = read();
  let m: usize = read();
  let mut ans = vec![0; m];
  let mut uf = UnionFind::new(n);

  let mut ab: Vec<(usize, usize)> = vec![];
  for _ in 0..m {
    let a: usize = read();
    let b: usize = read();
    ab.push((a, b));
  }

  ans[m - 1] = n * (n - 1) / 2;
  for i in (1..m).rev() {
    let (a, b) = ab[i];
    let (a, b) = (a - 1, b - 1);
    let na = uf.size(a);
    let nb = uf.size(b);
    if uf.union(a, b) {
      ans[i - 1] = ans[i] - na * nb;
    } else {
      ans[i - 1] = ans[i];
    }
  }

  for a in ans {
    println!("{}", a);
  }
}


struct UnionFind {
  parent: Vec<usize>,
  size: Vec<usize>,
}
impl UnionFind {
  fn new(n: usize) -> UnionFind {
    UnionFind {
      parent: (0..n).collect(),
      size: vec![1; n],
    }
  }
  fn find(&mut self, x: usize) -> usize {
    if self.parent[x] == x {
      x
    } else {
      let p = self.parent[x];
      self.parent[x] = self.find(p);
      self.parent[x]
    }
  }
  fn union(&mut self, x: usize, y: usize) -> bool {
    let mut x = self.find(x);
    let mut y = self.find(y);
    if x == y {
      false
    } else {
      if self.size[x] < self.size[y] {
        std::mem::swap(&mut x, &mut y);
      }
      self.parent[y] = x;
      self.size[x] += self.size[y];
      true
    }
  }
  fn size(&mut self, x: usize) -> usize {
    let p = self.find(x);
    self.size[p]
  }
}
