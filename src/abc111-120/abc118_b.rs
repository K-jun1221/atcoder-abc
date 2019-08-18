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
  let m = read::<i32>();
  let mut votes = vec![0; m as usize];

  for _ in 0..n {
    let k = read::<i32>();
    for _ in 0..k {
      let a = read::<i32>();
      votes[(a - 1) as usize] += 1;
    }
  }

  let mut answer = 0;
  for v in votes {
    if v == n {
      answer += 1
    }
  }

  println!("{:?}", answer);

}

