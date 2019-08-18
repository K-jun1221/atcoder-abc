
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
use std::collections::HashSet;

fn main() {
  let x = read::<i64>();
  let y = read::<i64>();
  let mut memo: Vec<Vec<i64>> = vec![vec![0, 0]; ((y / 1000) - x + 20) as usize];

  if y / 1000 == x {
    println!("{} {} {}", 0, 0, y / 1000);
  } else {
    let mut four_power = 0;
    while four_power <= (y / 1000 - x) && four_power / 4 <= x {
      let mut nine_power = 0;
      nine_power += four_power;
      memo[four_power as usize] = vec![four_power, nine_power];
      while nine_power <= (y / 1000 - x) && (nine_power - four_power) / 9 + (four_power / 4) <= x {
        memo[nine_power as usize] = vec![four_power, nine_power];
        nine_power += 9;
      }
      four_power = four_power + 4;
    }

    if memo[((y / 1000) - x) as usize] != vec![0, 0] {
      let higuchi = memo[((y / 1000) - x) as usize][0] / 4;
      let yukichi = (memo[((y / 1000) - x) as usize][1] - memo[((y / 1000) - x) as usize][0]) / 9;
      println!("{} {} {}", yukichi, higuchi, (x - higuchi - yukichi));
    } else {
      println!("{} {} {}", -1, -1, -1);
    }
  }
}

