// mod custom_lib;
use std::collections::HashMap;

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
  let n = read::<u32>();
  let m = read::<u32>();

  let mut s_map: HashMap<u32, Vec<u32>> = HashMap::new();
  let light_status = vec![0; m as usize];
  let mut answer = 0;

  for k_index in 0..m {
    let k = read::<u32>();
    let conn_switch = read_n::<u32>(k);
    for switch in conn_switch {
      if s_map.contains_key(&switch) {
        let mut prev_vec = s_map.get_mut(&switch).unwrap();
        prev_vec.push(k_index);
      } else {
        s_map.insert(switch, vec![k_index]);
      }
    }
  }

  let idial_light_status = read_n::<u32>(m);


  let two: i32 = 2;
  for i in 0..two.pow(n) {
    let result: String = format!("{:0>keta$b}", i, keta = n as usize);
    let mut clone_status = light_status.clone();

    for (keta, value) in result.chars().enumerate() {
      if value == '1' {
        search(&mut clone_status, (keta + 1) as u32, &s_map);
      }
    }

    if check_the_same(&clone_status, &idial_light_status) {
      answer += 1
    };
  }

  println!("{}", answer);
}

fn search(current_status: &mut Vec<u32>, change_switch: u32, s_map: &HashMap<u32, Vec<u32>>) {
  let target_lights = match s_map.get(&change_switch) {
    Some(s) => s,
    None => return,
  };

  for light in target_lights {
    current_status[*light as usize] = if current_status[*light as usize] == 0 {
      1
    } else {
      0
    };
  }
}

fn check_the_same(c_status: &Vec<u32>, i_status: &Vec<u32>) -> bool {
  let c_status_str: String = c_status
    .into_iter()
    .map(|i| i.to_string())
    .collect::<String>();
  let i_status_str: String = i_status
    .into_iter()
    .map(|i| i.to_string())
    .collect::<String>();
  if i_status_str == c_status_str {
    true
  } else {
    false
  }
}
