
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
use std::collections::HashMap;


fn main() {
  let x = read::<i64>();
  let mut dist: HashMap<char, i64> = HashMap::new();
  dist.insert('M', 0);
  dist.insert('A', 0);
  dist.insert('R', 0);
  dist.insert('C', 0);
  dist.insert('H', 0);
  for _ in 0..x {
    let y = read::<String>();
    let first = &y.chars().nth(0).unwrap();
    if *first == 'M' || *first == 'A' || *first == 'R' || *first == 'C' || *first == 'H' {
      let current = dist.entry(*first).or_insert(0);
      *current += 1;
    }
  }

  let mut total = 0;
  // MAR
  total += dist.get(&'M').unwrap() * dist.get(&'A').unwrap() * dist.get(&'R').unwrap();
  // MAC
  total += dist.get(&'M').unwrap() * dist.get(&'A').unwrap() * dist.get(&'C').unwrap();
  // MAH
  total += dist.get(&'M').unwrap() * dist.get(&'A').unwrap() * dist.get(&'H').unwrap();
  // MRC
  total += dist.get(&'M').unwrap() * dist.get(&'C').unwrap() * dist.get(&'R').unwrap();
  // MRH
  total += dist.get(&'M').unwrap() * dist.get(&'H').unwrap() * dist.get(&'R').unwrap();
  // MCH
  total += dist.get(&'M').unwrap() * dist.get(&'C').unwrap() * dist.get(&'H').unwrap();
  // ARC
  total += dist.get(&'R').unwrap() * dist.get(&'A').unwrap() * dist.get(&'C').unwrap();
  // ARH
  total += dist.get(&'H').unwrap() * dist.get(&'A').unwrap() * dist.get(&'R').unwrap();
  // ACH
  total += dist.get(&'C').unwrap() * dist.get(&'A').unwrap() * dist.get(&'H').unwrap();
  // RCH
  total += dist.get(&'C').unwrap() * dist.get(&'H').unwrap() * dist.get(&'R').unwrap();

  println!("{}", total);
}
