## HashSet
```rust
use std::collections::HashSet;

let mut uniq = HashSet::new();
```

- `.insert(a)`
- `.remove(a)`
- `.len()`

## HashMap

```rust
use std::collections::HashMap;

let mut dist: HashMap<type, type> = HashMap::new();

for (k, v) in dist.iter() {}
if dist.contains_key(&t) {
  let current = dist.get_mut(&t).unwrap();
  current.push(d);
} else {
  dist.insert(t, vec![d]);
}
```

- `.entry(a).or_insert(b);`
- `.len()`
- `.values().all(|x| *x == 2)`
- `get_mut(a).unwrap()`
- `insert(a, b);`
- `contains_key()`

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

## 型変換一覧

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

## コンビネーション(nCr)
https://docs.rs/reform/0.1.0/src/reform/tools.rs.html#285-298
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
fn combination(a: i64, b: i64) -> i64 {
    let mut ret = 1;
    for i in 0..b {
        ret = ret * (a - i) % MODULO;
    }
    return ret * inverse(fact(b)) % MODULO;
}

fn inverse(x: i64) -> i64 {
    powmod(x, MODULO - 2)
}

#[test]
fn test_combination() {
    assert_eq!(combination(6, 3), (6 * 5 * 4) / (3 * 2 * 1));
    assert_eq!(fact(5), 5 * 4 * 3 * 2 * 1);
    assert_eq!(combination(4, 1), 4);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(powmod(2, 4), 16);
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
    j += 1;
  }
  i += 1;
  j = 0;
}
```

## グラフ理論

隣接行列としてまとめた方がはるかに扱いやすい。その後のDFSとかの実装もめちゃくちゃ楽になる。
隣接行列はデータ量が圧倒的に多くなるのが欠点。

隣接行列 Ver
```rust
// n: node, m: edge
fn main() {
  let n = read::<usize>();
  let m = read::<usize>();
  let mut map: Vec<Vec<usize>> = vec![vec![0; n]; n];
  for _ in 0..m {
    let a = read::<usize>();
    let b = read::<usize>();
    map[a - 1][b - 1] = 1;
    map[b - 1][a - 1] = 1;
  }
  
  let mut visited: Vec<usize> = vec![];
  let mut node = 0;
  
  dfs(node, &mut visited, &map);
}



fn dfs(node: usize, visited: &mut Vec<usize>, map: &Vec<Vec<usize>>) {
  if visited.contains(&node) {
    return;
  }
  visited.push(node);
  for (i, v) in map[node].iter().enumerate() {
    if *v == 1 {
      dfs(i, visited, map)
    }
  }
}

```

HashMap Ver
```rust
fn main() {
  let n = read::<i64>();
  let m = read::<i64>();
  let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

  for _ in 0..m {
    let a = read::<i64>();
    let b = read::<i64>();
    if map.contains_key(&a) {
      let current = map.get_mut(&a).unwrap();
      current.push(b)
    } else {
      map.insert(a, vec![b]);
    }

    if map.contains_key(&b) {
      let current = map.get_mut(&b).unwrap();
      current.push(a)
    } else {
      map.insert(b, vec![a]);
    }
  }

  let mut visited = vec![];
  let mut node = 1;
  let mut deep = 1;

  dfs(node, &mut visited, &map, deep);

  if visited.contains(&(n)) {
    println!("POSSIBLE");
  } else {
    println!("IMPOSSIBLE");
  }
}

fn dfs(node: i64, visited: &mut Vec<i64>, map: &HashMap<i64, Vec<i64>>, deep: i64) {
  // println!("node: {}", node);
  if 3 < deep {
    return;
  }
  if visited.contains(&node) {
    return;
  }
  visited.push(node);
  if map.contains_key(&node) {
    for (i, v) in map.get(&node).unwrap().iter().enumerate() {
      dfs(*v as i64, visited, map, deep + 1)
    }
  }

}
```

## 2^10の列挙

```rust
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
```

- `.find(a)` 同じグループに属する番号を得る
- `.union(a, b)` グループを結合する。すでに結合している場合にはFalseが返る
- `.size(a)` グループのサイズを得る


### 公式Docs

- TraitはGoのInterfaceみたいなもの
- 基本的にはStringは&str.to_string()で作成する。
- VectorはForで回したときには参照を取っている。
- for i in &mut でミュータブルな値を取得可能になるかも?
