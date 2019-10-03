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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

fn main() {
  let n: usize = read();
  let a: Vec<i64> = read_n(n);
  let b: Vec<i64> = read_n(n);
  let c: Vec<i64> = read_n(n-1);

  // println!("{:?}", a);
  // println!("{:?}", b);
  // println!("{:?}", c);

  let mut prev = -1;
  let mut ans = 0;

  for i in a {
    ans += b[(i-1) as usize];

    if prev + 1 == i {
      ans += c[(prev-1) as usize];
    }

    prev = i;
  }

  println!("{}", ans);
}
