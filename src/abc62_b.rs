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

use std::cmp::min;

fn main() {
    let h: usize = read();
    let w: usize = read();
    let mut map: Vec<Vec<char>> = vec![vec!['#'; w + 2]; h + 2];

    for i in 1..h + 1 {
        let row: String = read();
        let mut rowc: Vec<char> = row.chars().collect();
        for w in 0..rowc.len() {
            map[i][w + 1] = rowc[w];
        }
    }

    for i in 0..map.len() {
        let mi = &map[i];
        let mut row: String = "".to_string();
        for i in mi {
            row += &i.to_string();
        }
        println!("{}", row);
    }
}
