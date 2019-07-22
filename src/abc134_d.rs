
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

use std::collections::HashSet;

fn divisors(N: i32) -> HashSet<i32> {
  let mut ds = vec![];
  let mut d = 1;
  while d * d <= N {
    if N % d == 0 {
      ds.push(d);
      ds.push(N / d);
    }
    d += 1;
  }
  let uniq: HashSet<i32> = ds.into_iter().collect();
  uniq
}

use std::cmp::{max, min};
use std::collections::BinaryHeap;

fn main() {
  let n = read::<i32>();
  let mut memo = vec![];

  for _ in 0..n {
    let a_i = read::<i32>();
    memo.push(a_i);
  }

  // println!("memo:{:?}", memo);

  for i in 0..n {
    let rev_i = (n - i - 1) as usize;
    let rev_num = n - i;

    if (memo[rev_i]) % 2 == 1 {
      let yakusuu = divisors(rev_num);
      // println!("yakusuu: {:?}", yakusuu);
      for j in yakusuu {
        if j != rev_num {
          memo[(j - 1) as usize] = (memo[(j - 1) as usize] + 1) % 2;
        }
      }
      // println!("{:?}", memo);
    }
  }

  // println!("memo: {:?}", memo);

  let mut ans = String::from("");
  let mut sum = 0;


  for (i, v) in memo.iter().enumerate() {
    if *v != 0 {
      sum += 1;
      ans.push_str(&(i + 1).to_string());
      if i + 1 != n as usize {
        ans.push(' ');
      }
    }
  }

  // TODO 合成を出す
  println!("{}", sum);
  if sum != 0 {
    println!("{}", ans);
  }

}

