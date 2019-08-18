
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

struct Block {
  x: i32,
  y: i32,
  h: i32,
}

fn main() {
  let n = read::<i32>();
  let mut b = vec![];
  for _ in 0..n {
    let x = read::<i32>();
    let y = read::<i32>();
    let h = read::<i32>();
    b.push(Block { x: x, y: y, h: h })
  }

  let mut done = false;
  let mut ans_h = 0;
  let mut ans_x = 0;
  let mut ans_y = 0;

  for x_i in 0..101 {
    for y_i in 0..101 {
      let h_i = b[0].h + (x_i - b[0].x).abs() + (y_i - b[0].y).abs();
      if h_i <= 0 {
        continue;
      }

      let mut is_ok = true;

      for block in &b {
        if h_i != block.h + (x_i - block.x).abs() + (y_i - block.y).abs() {
          is_ok = false
        }
      }

      if is_ok {
        ans_h = h_i;
        ans_x = x_i;
        ans_y = y_i;
        done = true;
        break;
      }
    }
    if done {
      break;
    }
  }

  println!("{} {} {}", ans_y, ans_x, ans_h);
}

