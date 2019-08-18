
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
  let h = read::<i64>();
  let w = read::<i64>();
  let mut map: Vec<Vec<char>> = vec![];

  for i in 0..h {
    let row = read::<String>();
    map.push(row.chars().collect())
  }

  for i in 0..h {
    for j in 0..w {
      let i_size = i as usize;
      let j_size = j as usize;
      let mut cnt = 0;
      if map[i_size][j_size] == '#' {
        continue;
      }

      // 左上
      if 0 < i && 0 < j && map[i_size - 1][j_size - 1] == '#' {
        cnt += 1;
      }
      // 上
      if 0 < i && map[i_size - 1][j_size] == '#' {
        cnt += 1;
      }
      // 右上
      if 0 < i && j + 1 < w && map[i_size - 1][j_size + 1] == '#' {
        cnt += 1;
      }
      // 左
      if 0 < j && map[i_size][j_size - 1] == '#' {
        cnt += 1;
      }
      // 右
      if j + 1 < w && map[i_size][j_size + 1] == '#' {
        cnt += 1;
      }
      // 左下
      if i + 1 < h && 0 < j && map[i_size + 1][j_size - 1] == '#' {
        cnt += 1;
      }
      // 下
      if i + 1 < h && map[i_size + 1][j_size] == '#' {
        cnt += 1;
      }
      // 右下
      if i + 1 < h && j + 1 < w && map[i_size + 1][j_size + 1] == '#' {
        cnt += 1;
      }
      map[i_size][j_size] = cnt.to_string().chars().nth(0).unwrap();
    }
  }

  for i in map {
    let mut s = String::from("");
    for j in i.iter() {
      s.push_str(&j.to_string());
    }
    println!("{}", s);
  }
}

