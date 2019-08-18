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
  let mut a = vec![];
  let mut b = vec![];
  let mut ans = 0;

  for _ in 0..n + 1 {
    let a_i = read::<i64>();
    a.push(a_i)
  }
  for _ in 0..n {
    let b_i = read::<i64>();
    b.push(b_i)
  }

  // b[n]
  for i in 0..n {
    let idx = (n - i - 1) as usize;
    if a[idx + 1] >= b[idx] {
      ans += b[idx];
      a[idx + 1] = a[idx + 1] - b[idx];
    } else {
      ans += a[idx + 1];
      b[idx] = b[idx] - a[idx + 1];


      if a[idx] >= b[idx] {
        ans += b[idx];
        a[idx] = a[idx] - b[idx];
      } else {
        ans += a[idx];
        a[idx] = 0;

      }
    }
  }

  println!("{:?}", ans);
}

