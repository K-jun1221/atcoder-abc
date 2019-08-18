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

use std::cmp::{max, min};
use std::collections::VecDeque;


fn main() {
  let n: usize = read();
  let m: usize = read();
  let m_vec: Vec<usize> = read_n(m);
  let nums = [10, 2, 5, 5, 4, 5, 6, 3, 7, 6];
  // [桁数, 最後の数]
  const INF: i32 = 1001001001;

  let nums = [10, 2, 5, 5, 4, 5, 6, 3, 7, 6];
  let mut dp = vec![-INF; n + 1];
  dp[0] = 0;
  for i in 1..n + 1 {
    for &a in &m_vec {
      if nums[a] <= i {
        dp[i] = std::cmp::max(dp[i], dp[i - nums[a]] + 1);
      }
    }
  }

  let mx = dp[n] as usize;

  let mut rem = n;
  let mut ans = vec![0; mx];
  for i in 0..mx {
    for &a in &m_vec {
      if rem >= nums[a] && dp[rem - nums[a]] == dp[rem] - 1 {
        ans[i] = std::cmp::max(ans[i], a);
      }
    }
    rem -= nums[ans[i]];
  }
  ans.sort_by_key(|&a| -(a as i32));


  // println!("{:?}", ans);
  for i in ans {
    print!("{}", i)
  }

  // for i in m_vec {
  //   dist.entry().or_insert(b);
  // }

}
