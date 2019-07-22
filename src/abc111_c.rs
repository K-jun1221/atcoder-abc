
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

use std::collections::HashMap;


fn main() {
  let n = read::<i32>();
  let mut even = HashMap::new();
  let mut odd = HashMap::new();

  for i in 1..n + 1 {
    let item = read::<u32>();
    if i % 2 == 0 {
      *even.entry(item).or_insert(0) += 1;
    } else {
      *odd.entry(item).or_insert(0) += 1;
    }
  }
  let even_max = get_max_key_value(&even);
  let odd_max = get_max_key_value(&odd);

  if even_max.0 != odd_max.0 {
    println!("{}", n as u32 - even_max.1 - odd_max.1);
  } else {
    even.remove(&even_max.0);
    odd.remove(&odd_max.0);

    let even_2nd_max = get_max_key_value(&even);
    let odd_2nd_max = get_max_key_value(&odd);
    if even_2nd_max.1 > odd_2nd_max.1 {
      println!("{}", n as u32 - odd_max.1 - even_2nd_max.1);
    } else {
      println!("{}", n as u32 - even_max.1 - odd_2nd_max.1);
    }
  }
}

fn get_max_key_value(map: &HashMap<u32, u32>) -> (u32, u32) {
  let mut max = (0, 0);
  for (key, value) in map {
    if max.1 < *value {
      max = (*key, *value);
    }
  }
  max
}