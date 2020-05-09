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
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let s: Vec<char> = read::<String>().chars().collect();
        map.push(s);
    }
    let mut ans: String = "".to_string();

    let abs: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for c in abs {
        let min_cnt = map
            .iter()
            .map(|x| x.iter().filter(|&x| x == &c).count())
            .min()
            .unwrap();
        for _ in 0..min_cnt {
            ans += &c.to_string();
        }
    }
    println!("{}", ans);
}
