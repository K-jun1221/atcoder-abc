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
  let n = read::<i32>();
  let mut value = vec![];
  let mut cost = vec![];


  for _ in 0..n {
    let v = read::<i32>();
    value.push(v)
  }
  for _ in 0..n {
    let c = read::<i32>();
    cost.push(c)
  }

  let mut answer = 0;
  for i in 0..n {
    let v = value[i as usize] - cost[i as usize];
    if v > 0 {
      answer += v
    }
  }

  println!("{}", answer);
}

