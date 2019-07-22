
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
  let n = read::<i32>();
  let m = read::<i32>();
  let x = read::<i32>();
  let y = read::<i32>();

  let mut y_v = vec![];
  let mut x_v = vec![];

  for _ in 0..n {
    let x_i = read::<i32>();
    x_v.push(x_i)
  }

  for _ in 0..m {
    let y_i = read::<i32>();
    y_v.push(y_i)
  }


  let y_v_min = y_v.iter().min().unwrap();
  let x_v_max = x_v.iter().max().unwrap();

  let mut z_v = vec![];
  for i in x + 1..y + 1 {
    z_v.push(i as i32)
  }

  let mut ok = false;

  for z_i in z_v {
    if z_i <= *y_v_min && z_i > *x_v_max {
      ok = true;
      break;
    }
  }

  if ok {
    println!("No War");
  } else {
    println!("War");
  }
}

