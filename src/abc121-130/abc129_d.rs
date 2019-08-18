// mod custom_lib;
use std::cmp;

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

pub fn read_n<T: FromStr>(n: u32) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: u32, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}


fn main() {
  let h = read::<usize>();
  let w = read::<u32>();

  let mut s = Vec::new();
  let mut s_point = Vec::new();

  for _ in 0..h {
    let mut row = Vec::new();
    for c in read::<String>().chars() {
      row.push(c);
    }
    s_point.push(calc_width_lamb(&row));
    s.push(row);
  }

  for w_idx in 0..w {
    let height_point = calc_height_lamb(&s, w_idx as usize);
    for (h_idx, v) in height_point.iter().enumerate() {
      if v != &0 {
        s_point[h_idx as usize][w_idx as usize] += v - 1;
      }
    }
  }

  // 結果
  let mut answer = 0;
  for item in s_point {
    answer = cmp::max(*item.iter().max().unwrap(), answer)
  }

  println!("{}", answer);
}

fn calc_height_lamb(map: &Vec<Vec<char>>, w_idx: usize) -> Vec<u32> {
  let mut height_row = vec![];
  for row in map {
    height_row.push(row[w_idx])
  }
  calc_width_lamb(&height_row)
}

fn calc_width_lamb(row: &Vec<char>) -> Vec<u32> {
  let mut row_copy = row.clone();
  row_copy.insert(0, '#');
  let last_idx = row_copy.len();
  row_copy.insert(last_idx, '#');

  let mut prev_sharp = 0;
  let mut result_vec = vec![0; row_copy.len()];

  for (i, v) in row_copy.iter().enumerate() {
    if v == &'#' {
      for idx in prev_sharp..i {
        result_vec[idx] = (i - prev_sharp) as u32;
      }
      prev_sharp = i + 1;
    }
  }

  result_vec.remove(0);
  result_vec.remove(last_idx - 1);
  result_vec
}
