
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
  let n = read::<usize>();
  let q = read::<i32>();
  let s = read::<String>();
  let mut a: Vec<(i32, i32)> = vec![];
  let mut memo = vec![0; n];
  let mut count = 0;

  for _ in 0..q {
    let start = read::<i32>();
    let end = read::<i32>();
    a.push((start, end))
  }

  let mut prev_char: char = '#';

  for (k, v) in s.chars().enumerate() {
    if v == 'C' && prev_char == 'A' {
      count += 1;
    }

    memo[k] = count;
    prev_char = v;
  }

  for (s, e) in a {
    println!("{:?}", memo[(e - 1) as usize] - memo[(s - 1) as usize]);
  }

}

