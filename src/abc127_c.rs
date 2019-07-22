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

use std::collections::BinaryHeap;

fn main() {

  let n = read::<u32>();
  let m = read::<u32>();

  let mut start_gates = BinaryHeap::new();
  let mut end_gates = BinaryHeap::new();

  for _ in 0..m {
    let start = read::<i32>();
    let end = read::<i32>();
    start_gates.push(start);
    end_gates.push(-end);
  }

  let sg = start_gates.pop().unwrap();
  let eg = end_gates.pop().unwrap();
  let answer = (-eg) - sg + 1;

  if answer > 0 {
    println!("{}", answer);
  } else {
    println!("{}", 0);
  }

}
