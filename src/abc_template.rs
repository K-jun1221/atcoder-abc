mod custom_lib;

fn main() {
  let h = custom_lib::read::<usize>();
  let w = custom_lib::read::<u32>();

  let mut s = Vec::new();
  for _ in 0..h {
    let mut row = Vec::new();
    for c in custom_lib::read::<String>().chars() {
      row.push(c);
    }
    s.push(row);
  }

  println!("{:?}", s);
}

// mod custom_lib;
// BTreeMapを用いた実装例
use std::collections::BTreeMap;

fn main() {
  let n = custom_lib::read::<usize>();
  let mut map: BTreeMap<String, BTreeMap<u32, usize>> = BTreeMap::new();

  for i in 0..n {
    let s = custom_lib::read::<String>();
    let p = custom_lib::read::<u32>();

    if map.contains_key(&s) {
      let mut prev_vec = map.get_mut(&s).unwrap();
      prev_vec.insert(p, i + 1);
    } else {
      let mut item_map = BTreeMap::new();
      item_map.insert(p, i + 1);
      map.insert(s, item_map);
    }
  }

  for (_, v) in &map {
    for (_, index) in v.iter().rev() {
      println!("{}", index)
    }
  }
}

// 深さ優先全探索
// let two: i32 = 2;
//   for i in 0..two.pow(n) {
//     let result: String = format!("{:0>keta$b}", i, keta = n as usize);
//     let mut clone_status = light_status.clone();

// グラフの全探索
// abc126_d

// 最大公約数
fn gcd(a: usize, b: usize) -> usize {
  if a % b == 0 {
    b
  } else {
    gcd(b, a % b)
  }
}

// 最小公倍数
fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

// 優先度付きキュー
use std::collections::BinaryHeap;
let mut bh_a: BinaryHeap<i64> = BinaryHeap::new();
// bh_a.push()
// bh_a.pop()

// 二分探索
// binary_search
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::from(vec![1, 2, 4, 5, 7]);
heap.push(6);
heap.push(3);

let vec = heap.into_sorted_vec();
assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
println!("{:?}", vec.binary_search(&4) );


// ハッシュマップを探索
// fn get_max_key_value(map: &HashMap<u32, u32>) -> (u32, u32){
//     let mut max = (0, 0);
//     for (key, value) in map{
//         if max.1 < *value{
//             max = (*key, *value);
//         }
//     }
//     max
// }

// String to Vec<char>
// let a_v = a.chars().collect::<Vec<char>>();


  // let new_cells: Vec<char> = cells
  //   .iter()
  //   .filter(|v| {
  //     let s: String = v.iter().collect::<String>();
  //     s == c
  //   })
  //   .collect();

// 約数を列挙
use std::collections::HashSet;

fn divisors(N: i32) -> HashSet<i32> {
  let mut ds = vec![];
  let mut d = 1;
  while d * d <= N {
    if N % d == 0 {
      ds.push(d);
      ds.push(N / d);
    }
    d += 1;
  }
  let uniq: HashSet<i32> = ds.into_iter().collect();
  uniq
}