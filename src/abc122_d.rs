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

use std::cmp::{max, min};

fn main() {
  let n = read::<i64>();
  let ten = 10 as u32;
  let MOD = ten.pow(9) + 7;
  let mut ans = 0;

  let mut dp = vec![vec![vec![vec![0; 4]; 4]; 4]; n as usize + 1];

  dp[0][3][3][3] = 1;
  // i文字目
  for i in 0..n {
    // 最後から3文字目
    for j in 0..4 {
      // 最後から2文字目
      for k in 0..4 {
        // 最後から1文字目
        for l in 0..4 {
          // println!("i: {}, j: {}, k {}, l: {}", i, j, k, l);
          if dp[i as usize][j][k][l] == 0 {
            continue;
          }


          for new in 0..4 {
            if k == 0 && l == 2 && new == 1 {
              continue;
            }
            if k == 2 && l == 0 && new == 1 {
              continue;
            }
            if j == 0 && l == 2 && new == 1 {
              continue;
            }
            if j == 0 && k == 2 && new == 1 {
              continue;
            }
            if k == 0 && l == 1 && new == 2 {
              continue;
            }

            dp[i as usize + 1][k][l][new] += dp[i as usize][j][k][l];
            dp[i as usize + 1][k][l][new] %= MOD;
          }
        }
      }
    }
  }

  let mut ans = 0;
  for i in &dp[n as usize] {
    for j in i {
      for k in j {
        ans += *k;
        ans %= MOD;
      }
    }
  }

  println!("{}", ans);
}

