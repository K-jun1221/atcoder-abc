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
    let n: usize = read();
    let m: usize = read();
    let mut map: Vec<Vec<char>> = vec![];
    let mut temp: Vec<Vec<char>> = vec![];

    for _ in 0..n {
        let row: String = read();
        let rowc: Vec<char> = row.chars().collect();
        map.push(rowc);
    }
    for _ in 0..m {
        let row: String = read();
        let rowc: Vec<char> = row.chars().collect();
        temp.push(rowc)
    }
    let mut ans = "No";

    'outermost: for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            let mut flag = false;
            'outer: for ii in i..i + m {
                for jj in j..j + m {
                    // println!("ii{}, jj{}", ii, jj);
                    if map[ii][jj] != temp[ii - i][jj - j] {
                        flag = true;
                        break 'outer;
                    }
                }
            }

            if flag == false {
                ans = "Yes";
                break 'outermost;
            }
        }
    }
    println!("{}", ans);
}
