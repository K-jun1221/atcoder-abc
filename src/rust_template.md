
## Starter Template

```rust

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

fn main() {

}

```

## 最大公約数

```rust
fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}
```

## 最小公倍数

```rust
fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}

fn lcm(x: i64, y: i64) -> i64 {
    x * (y / gcd(x, y))
}
```

```
## 高速で2^nを計算

```rust
fn powmod(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        let k = powmod(x, n / 2);
        return (k * k) % MODULO;
    } else {
        let k = powmod(x, n / 2);
        return (((k * k) % MODULO) * x) % MODULO;
    }
}
```

## 約数を列挙

```rust
use std::collections::HashSet;

fn divisors(n: i64) -> HashSet<i64> {
  let mut ds = vec![];
  let mut d = 1;
  while d * d <= n {
    if n % d == 0 {
      ds.push(d);
      ds.push(n / d);
    }
    d += 1;
  }
  let uniq: HashSet<i64> = ds.into_iter().collect();
  uniq
}
```

## convination(ncr)
https://docs.rs/reform/0.1.0/src/reform/tools.rs.html#285-298
```rust
pub struct Comb {
    max: usize,
    m: i64,
    fac: Vec<i64>,
    finv: Vec<i64>,
}
impl Comb {
    /// initialize table
    /// max: maximum that n as nCk takes
    /// m: MOD, a prime number
    /// O(n)
    pub fn new(max: usize, m: i64) -> Comb {
        let n = max;
        let mut fac = Vec::with_capacity(n);
        fac.push(1);
        fac.push(1);
        let mut finv = Vec::with_capacity(n);
        finv.push(1);
        finv.push(1);
        let mut inv = Vec::with_capacity(n);
        inv.push(1);
        inv.push(1);
        for i in 2..n {
            let i = i as i64;
            let tmp = fac.last().unwrap() * i % m;
            fac.push(tmp);
            let tmp = m - inv[(m % i) as usize] * (m / i) % m;
            inv.push(tmp);
            let tmp = finv.last().unwrap() * inv.last().unwrap() % m;
            finv.push(tmp);
        }
        Comb {
            max: max,
            m: m,
            fac: fac,
            finv: finv,
        }
    }
    /// nCr
    pub fn ncr(&self, n: usize, k: usize) -> i64 {
        // since n, k are usize, do not need to check `n < 0 || k < 0`
        if n < k || n > self.max {
            panic!("Invalid query (n, k) = ({}, {})", n, k);
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m) % self.m
        }
    }
}
```



## 2^10の列挙

```rust
<!-- too slow -->
let result: String = format!("{:0>keta$b}", i, keta = d as usize);
```

## BitDB

```rust
// 1101という二進数を10進数に変換する
let ay := [4, 3, 1];
let mut s = 0;
for t in ay {
  s += 1 << (t - 1);
}
// s = 13

// `|`  -> パターンOR
13 | 3 = 15
// 1101 or 10 = 1111


```

## Upper/Lower Bound

upper: val以下の数の個数を返す
lower: val未満の数の個数を返す

```rust
use std::cmp::Ordering;

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
```

##Union-Find 

```rust
let mut uf = UnionFind::new(n);

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        // initialize
        let mut v: Vec<usize> = vec![];
        for i in 0..n + 1 {
            v.push(i)
        }
        let mut s: Vec<usize> = vec![1; n + 1];

        UnionFind { parent: v, size: s }
    }

    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        } else {
            return self.root(self.parent[x]);
        }
    }

    fn size(&self, a: usize) -> usize {
        let ra = self.root(a);
        self.size[ra]
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb)
        };
        self.size[ra] += self.size[rb];
        self.parent[rb] = ra;
    }

    fn same(&self, a: usize, b: usize) -> bool {
        let ra = self.root(a);
        let rb = self.root(b);
        ra == rb
    }
}
```
## segment-tree
```
struct SegTree {
    op: fn(i64, i64) -> i64,
    pub v: Vec<i64>,
    next_two: usize,
}

impl SegTree {
    fn new(org: Vec<i64>, func: fn(i64, i64) -> i64) -> Self {
        let orglen = org.len();
        let mut next_two = 1;
        while next_two < orglen {
            next_two *= 2;
        }

        let vlen = next_two * 2;
        let mut v: Vec<i64> = vec![0; vlen];

        for i in (0..vlen).rev() {
            if next_two <= i && i - next_two < orglen {
                v[i] = org[i - next_two];
            }
            if i != 0 && i < next_two {
                v[i] = v[i * 2] | v[i * 2 + 1];
            }
        }
        SegTree { op: func, v: v, next_two: next_two }
    }
    fn update(&mut self, idx: usize, nv: i64) {
        self.v[idx + self.next_two] = nv;
        let mut current_idx = idx + self.next_two;
        while current_idx != 1 {
            current_idx = current_idx / 2;
            self.v[current_idx] = self.v[current_idx * 2] | self.v[current_idx * 2 + 1];
        }
    }
}

```

##degit dp
```
// ref: abc156_e.rs
```

## DFS, BFS

done.contains is too slow don't go in  this way

## next_permutation
```rust
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }

}
```
