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
  let mut a = vec![];
  let mut memo_l = vec![0; n];
  let mut memo_r = vec![0; n];
  let mut answers = vec![];

  for _ in 0..n {
    let v = read::<u32>();
    a.push(v);
  }

  let mut memo = a[a.len() - 1];
  for (i, v) in a.iter().rev().enumerate() {
    memo = gcd(memo, *v);
    memo_r[i] = memo;
  }

  let mut memo2 = a[0];
  for (i, v) in a.iter().enumerate() {
    memo2 = gcd(memo2, *v);
    memo_l[i] = memo2;
  }

  memo_r.reverse();
  memo_l.reverse();
  memo_r.remove(0);
  memo_l.remove(0);
  answers.push(memo_l[0]);
  answers.push(memo_r[0]);

  memo_r.remove(0);
  memo_l.remove(0);
  memo_r.reverse();
  memo_l.reverse();

  memo_r.reverse();

  for i in 0..memo_r.len() {
    answers.push(gcd(memo_r[i], memo_l[i]));
  }


  println!("{}", answers.iter().max().unwrap());

}

fn gcd(a: u32, b: u32) -> u32 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

