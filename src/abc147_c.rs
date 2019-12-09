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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        }
    }
}

use std::cmp::{max, min};

fn main() {
    let n: usize = read();

    let mut map: Vec<Vec<i64>> = vec![];

    for _ in 0..n {
        let a: i64 = read();
        let mut arr = vec![-1; n as usize];
        for _ in 0..a {
            let x: usize = read();
            let y: i64 = read();
            arr[x - 1] = y
        }
        map.push(arr);
    }

    for i in &map {
        // println!("{:?}", i);
    }

    let tow = 2 as u64;
    let mut tans = 0;

    for i in 0..(tow.pow(n as u32)) {
        let result: String = format!("{:0>keta$b}", i, keta = n as usize);
        // println!("{}", result);

        let result_char: Vec<char> = result.chars().collect();
        let mut ans = vec![-1; n];
        let mut res = true;
        for (idx, c) in result_char.iter().enumerate() {
            let i = c.to_digit(10).unwrap();
            let mut done = false;
            if i == 1 {
                // println!("{:?}", map[idx]);
                for (i2, j) in map[idx].iter().enumerate() {
                    if result_char[i2].to_digit(10).unwrap() != *j as u32 && *j != -1 {
                        done = true;
                        break;
                    }
                    if ans[i2] == -1 {
                        ans[i2] = *j
                    } else if ans[i2] != *j && *j != -1 {
                        done = true;
                        break;
                    }
                }
            }
            if done == true {
                res = false;
                break;
            }
        }
        if res == true {
            tans = max(tans, result_char.iter().filter(|&x| *x == '1').count());
        }
    }
    println!("{}", tans)
}
