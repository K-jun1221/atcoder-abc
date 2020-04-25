## HashSet
```rust
use std::collections::HashSet;

let mut uniq = HashSet::new();
```

- `.insert(a)`
- `.remove(a)`
- `.len()`

## HashMap

Vec<Vec<type>> is enough for this usage

## BinaryHeap

```rust
use std::collections::BinaryHeap;

let mut bh: BinaryHeap<type> = BinaryHeap::new();
```
- `BinaryHeap::from(vec![5, 7]);`
- `.push()`
- `.pop()`
- `BinaryHeap::from(vec![]);`

## BTreeMap

```rust
use std::collections::BTreeMap;

let mut map: BTreeMap<type, type> = BTreeMap::new();
for (k, v) in map.iter() {}
```

- `.entry(a).or_insert(b);`
- `.len()`
- `.values().all(|x| *x == 2)`
- `.get_mut()`
- `.insert()`

## VecDeque

```rust
use std::collections::VecDeque;

let mut v = VecDeque::new();
```

- `.push_front(a)`
- `.push_back(a)`
- `.pop_back(a)`
- `.pop_front(a)`

## type convertor

```rust
// int to string
i.to_string();

// char to int
c.to_digit(10).unwrap();

// char to string
c.to_string();

// chars to string
// let s: String = c.iter().collect()
// 1.15で動かん

// string to int
s.parse().unwrap();

// string to chars
s.chars().collect::<Vec<char>>();

// string to char
s.chars().nth(num).unwrap();

// string + &str
s.push_str(str)

// string + char
s.push(str)

// binary number to int
let ans = "101";
let intval = isize::from_str_radix(&ans, 2).unwrap();

// int to binary number
let num = format!("{:b}", i)
let result: String = format!("{:0>keta$b}", i, keta = d as 
```

## Vector

```rust
let mut vec = vec![];
```

- `.reverse()`
- `.clone()`
- `.split_off(a)`
- `.iter().map(|letter| { c += 1; (letter, c) })`
- `.iter().filter(|a| a.b == a.c)`

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

## 動的区画方(DP)

```rust
let mut dp: Vec<Vec<i64>> = vec![];
for _ in 0..height {
  dp.push(vec![0; width as usize + 1])
}

for i in 1..height + 1 {
  for j in 1..width + 1 {
    let i_size = i as usize;
    let j_size = j as usize;

    // ここにロジック
    dp[i_usize][j_usize] = min(
      dp[i_usize - 1][j_usize],
      dp[i_usize][j_usize - a as usize] + 1,
    );
  }
}
```


## 深さ優先全探索
```rust
let two: i64 = 2;
for i in 0..two.pow(n) {
  let result: String = format!("{:0>keta$b}", i, keta = n as usize);
  println("{}", result)
}
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

## Ai + Bj = X ( <= y )
上記の式を満たすXの列挙

O(n)
```rust

let mut dp = vec![false; y as usize + 1];

for i in a..y + 1 {
  let i_size = i as usize;
  dp[i_size] = if dp[i_size - a as usize] {
    true
  } else {
    false
  };
}

for j in b..f + 1 {
  let j_size = j as usize;
  dp[j_size] = if dp[j_size - b as usize] {
    true
  } else {
    false
  };
}
```

O(n^2)

```rust

let mut i = 0;
let mut j = 0;

while a * i <= y {
  while a * i + b * j <= y {
    println("i: {}, j: {}", i, j);
    j += 1; }
  i += 1;
  j = 0;
}
```

## graph

```rust
    type Graph = Vec<Vec<usize>>;

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

## Union-Find 

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
        self.parent[a] == self.parent[b]
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


