
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
use std::collections::HashMap;

fn main() {
  let n = read::<i64>();
  let m = read::<i64>();

  let mut map_n: Vec<String> = vec![];
  let mut map_m: Vec<String> = vec![];
  let mut ans = "No";

  for _ in 0..n {
    map_n.push(read::<String>())
  }

  for _ in 0..m {
    map_m.push(read::<String>())
  }

  for h in 0..n - m + 1 {
    for w in 0..n - m + 1 {
      // 検証
      let mut allright = true;
      for i in h..h + m {

        let w_size = w as usize;
        let m_size = m as usize;
        let row = &map_n[i as usize];
        if &map_m[(i - h) as usize] != &row[w_size..w_size + m_size] {
          allright = false;
        }
      }

      if allright {
        ans = "Yes";
        break;
      }
    }
    if ans == "Yes" {
      break;
    }
  }

  println!("{}", ans);
}

