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
use std::collections::VecDeque;

fn main() {
    let mut h: usize = read();
    let mut w: usize = read();
    let mut map: Vec<Vec<char>> = vec![];
    let mut ans = 0;

    for _ in 0..h {
        let row: String = read();
        let row_chars: Vec<char> = row.chars().collect();
        map.push(row_chars)
    }

    for i in 0..h * w {
        let sh = i / w;
        let sw = i % w;

        if &map[sh][sw] == &'#' {
            continue;
        }

        // dfs
        let longest = dfs(&map, h, w, sh, sw);
        // let longest = f(sh, sw, h, w, &map);
        ans = max(ans, longest);
    }
    println!("{}", ans);
}

fn bfs(map: &Vec<Vec<char>>, h: usize, w: usize, sh: usize, sw: usize) -> i64 {
    let mut check: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut ret = 0;
    for i2 in 0..h {
        for j2 in 0..w {
            if map[i2][j2] == '#' {
                check[i2][j2] = true;
            }
        }
    }
    let mut vd = VecDeque::new();
    vd.push_back((sh, sw, 0));
    check[sh][sw] = true;

    let mut ret = 0;
    vd.push_back((sh, sw, 0));

    loop {
        match vd.pop_front() {
            None => {
                break;
            }
            Some((sh2, sw2, d)) => {
                check[sh2][sw2] = true;
                if sh2 > 0 && !check[sh2 - 1][sw2] {
                    check[sh2 - 1][sw2] = true;
                    vd.push_back((sh2 - 1, sw2, d + 1));
                    ret = d + 1;
                }

                if sw2 > 0 && !check[sh2][sw2 - 1] {
                    check[sh2][sw2 - 1] = true;
                    vd.push_back((sh2, sw2 - 1, d + 1));
                    ret = d + 1;
                }
                if sh2 < h - 1 && !check[sh2 + 1][sw2] {
                    check[sh2 + 1][sw2] = true;
                    vd.push_back((sh2 + 1, sw2, d + 1));
                    ret = d + 1;
                }
                if sw2 < w - 1 && !check[sh2][sw2 + 1] {
                    check[sh2][sw2 + 1] = true;
                    vd.push_back((sh2, sw2 + 1, d + 1));
                    ret = d + 1;
                }
            }
        }
    }
    return ret;
}

fn f(i: usize, j: usize, h: usize, w: usize, mat: &Vec<Vec<char>>) -> u32 {
    let mut check: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut ret = 0;
    for i2 in 0..h {
        for j2 in 0..w {
            if mat[i2][j2] == '#' {
                check[i2][j2] = true;
            }
        }
    }
    let mut vd = VecDeque::new();
    vd.push_back((i, j, 0));
    check[i][j] = true;
    loop {
        match vd.pop_front() {
            None => {
                break;
            }
            Some((i2, j2, d)) => {
                check[i2][j2] = true;
                if i2 > 0 && !check[i2 - 1][j2] {
                    check[i2 - 1][j2] = true;
                    vd.push_back((i2 - 1, j2, d + 1));
                    ret = d + 1;
                }
                if i2 < h - 1 && !check[i2 + 1][j2] {
                    check[i2 + 1][j2] = true;
                    vd.push_back((i2 + 1, j2, d + 1));
                    ret = d + 1;
                }
                if j2 > 0 && !check[i2][j2 - 1] {
                    check[i2][j2 - 1] = true;
                    vd.push_back((i2, j2 - 1, d + 1));
                    ret = d + 1;
                }
                if j2 < w - 1 && !check[i2][j2 + 1] {
                    check[i2][j2 + 1] = true;
                    vd.push_back((i2, j2 + 1, d + 1));
                    ret = d + 1;
                }
            }
        }
    }
    ret
}
