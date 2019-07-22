
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

  for _ in 0..n {
    let a_i = read::<i32>();
    a.push(a_i);
  }

  let base = a.iter().min().expect("faild to find min");

  println!("{}", dsf(&a[..], base) + base);
}

fn dsf(a_p: &[i32], base: &i32) -> i32 {
  let a_min = match a_p.iter().min() {
    Some(s) => s,
    None => return 0,
  };
  let min_idx = find_all(a_p, *a_min);

  if min_idx.len() == a_p.len() {
    return a_min - base;
  }

  let mut prev_i: usize = 0;
  let mut sum = 0;

  for i in &min_idx {
    let a_slice = &a_p[prev_i..*i];
    sum += dsf(a_slice, a_min);

    prev_i = i + 1;
  }

  if prev_i < a_p.len() {
    let a_slice = &a_p[prev_i..];
    sum += dsf(a_slice, a_min);
  }
  sum += (*a_min - base);
  sum
}

fn find_all(a_p: &[i32], t: i32) -> Vec<usize> {
  let mut a_is = vec![];
  for (i, v) in a_p.iter().enumerate() {
    if *v == t {
      a_is.push(i);
    }
  }
  return a_is;
}
