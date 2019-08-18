
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
  let h = read::<i32>();
  let w = read::<i32>();
  let mut cells: Vec<Vec<char>> = vec![];

  for _ in 0..h {
    let c = read::<String>();
    let c_vec = c.chars().collect::<Vec<char>>();
    cells.push(c_vec);
  }

  let mut check_1 = true;
  let mut check_2 = true;

  while check_1 || check_2 {
    check_1 = check_width(&mut cells);
    let (c, b) = check_height(&mut cells);
    cells = c;
    check_2 = b;
  }

  for i in cells {
    let mut row = String::new();
    for j in i {
      row.push(j)
    }
    println!("{}", row);
  }
}

fn check_width(cells: &mut Vec<Vec<char>>) -> bool {
  let c: Vec<char> = vec!['.'; cells[0].len()];
  while true {
    let pos = match cells.iter().position(|x| *x == c) {
      Some(s) => s,
      None => return false,
    };
    cells.remove(pos);
  }

  true
}

fn check_height(cells: &mut Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
  let mut rem_idx = vec![];

  for i in 0..cells[0].len() {
    rem_idx.push(i)
  }

  for i in 0..cells[0].len() {
    if cells.iter().all(|c| c[i] == '.') {
      rem_idx.remove(i);
    }
  }
  let mut new_v: Vec<Vec<char>> = vec![];
  for i in cells.iter() {
    new_v.push(rem_idx.iter().map(|&index| i[index]).collect());
  }

  (new_v, cells[0].len() > rem_idx.len())
}
