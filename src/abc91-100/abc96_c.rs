
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
  let h = read::<i64>();
  let w = read::<i64>();
  let mut map: Vec<Vec<char>> = vec![];
  let mut done = false;
  let mut ans = "Yes";

  for _ in 0..h {
    let row = read::<String>();
    map.push(row.chars().collect());
  }

  for h_i in 0..h {
    for w_i in 0..w {
      let h_idx = h_i as usize;
      let w_idx = w_i as usize;
      if map[h_idx][w_idx] == '#' {
        let mut ok = false;
        if h_i != 0 && map[h_idx - 1][w_idx] == '#' {
          ok = true
        }
        if h_i != h - 1 && map[h_idx + 1][w_idx] == '#' {
          ok = true
        }
        if w_i != w - 1 && map[h_idx][w_idx + 1] == '#' {
          ok = true
        }
        if w_i != 0 && map[h_idx][w_idx - 1] == '#' {
          ok = true
        }

        if !ok {
          ans = "No";
          done = true;
          break;
        }
      }
    }

    if done {
      break;
    }
  }

  println!("{}", ans);

}

