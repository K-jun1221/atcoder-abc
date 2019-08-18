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
  let c = read::<i32>();
  let mut b = vec![];
  let mut ans = 0;

  let mut a: Vec<Vec<i32>> = vec![];

  for _ in 0..m {
    let b_i = read::<i32>();
    b.push(b_i)
  }

  for _ in 0..n {
    let mut a_s = vec![];
    for _ in 0..m {
      let a_i = read::<i32>();
      a_s.push(a_i);
    }
    a.push(a_s);
  }

  for i in a {
    let mut result = 0;
    for j in 0..m {
      result += i[j as usize] * b[j as usize];
    }
    if result + c > 0 {
      ans += 1;
    }
  }

  println!("{:?}", ans);
}

