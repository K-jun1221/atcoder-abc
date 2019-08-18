
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

struct question {
  point: u32,
  stock: u32,
  c_point: u32,
}

fn main() {
  let d = read::<u32>();
  let g = read::<u32>();
  let mut qs = vec![];
  let mut answers = vec![];

  for i in 0..d {
    let p = read::<u32>();
    let c = read::<u32>();
    qs.push(question {
      point: (i + 1) * 100,
      stock: p,
      c_point: c,
    })
  }

  let two: u32 = 2;
  for i in 0..two.pow(d) {
    let result: String = format!("{:0>keta$b}", i, keta = d as usize);

    let r_char: Vec<char> = result.chars().collect();

    let yes_r: Vec<&question> = qs
      .iter()
      .filter(|x| r_char[((x.point / 100) - 1) as usize] == '1')
      .collect();
    let no_r: Vec<&question> = qs
      .iter()
      .filter(|x| r_char[((x.point / 100) - 1) as usize] != '1')
      .collect();

    let mut b_p: u32 = yes_r.iter().map(|x| (x.point * x.stock) + x.c_point).sum();
    let mut cost: u32 = yes_r.iter().map(|x| x.stock).sum();

    let mut no_r_cp = no_r.clone();

    if b_p >= g {
      answers.push(cost)
    } else {
      no_r_cp.sort_by_key(|x| x.point);
      let max_q: &question = match no_r_cp.last() {
        Some(s) => s,
        None => continue,
      };
      for _ in 1..max_q.stock {
        b_p += max_q.point;
        cost += 1;

        if b_p >= g {
          answers.push(cost)
        }
      }
    }
  }

  println!("{:?}", answers.iter().min().unwrap());
}

