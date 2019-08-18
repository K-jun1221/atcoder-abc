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
  let k = read::<i32>();

  let mut memo = vec![-1; n + 1];

  for i in 0..n {
    let mut index = i + 1;
    if memo[index] != -1 {
      continue;
    }

    let mut current: i32 = (i + 1) as i32;
    let mut count = 0;
    while current < k {
      current = current * 2;
      count += 1;
    }
    while count > 0 && index < n + 1 {
      memo[index] = count;
      index = index * 2;
      count -= 1;
    }
  }

  memo.remove(0);
  let mut answer: f64 = 0.0;

  for v in memo.iter() {
    if *v == -1 {
      answer += 1.0 / n as f64;
      continue;
    }

    let percent = (1.0 / n as f64) * (0.5 as f64).powf(*v as f64);
    answer += percent;
  }
  println!("{}", answer);
}
